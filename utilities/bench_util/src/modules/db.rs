use super::constants::MEASUREMENT_NAME;
use anyhow::{Context, Ok, anyhow};
use reqwest::blocking::Client;
use serde_json::{Map, Value};
use std::env;
use std::fmt::Write;
use std::fs::File;
use std::io::BufReader;

/// Dynamically extracts fields from a JSON `Value` using a dot-delimited path (e.g. "slope.confidence_interval.lower_bound")
fn get_nested_value<'a>(value: &'a Value, field_path: &str) -> Option<&'a Value> {
    let mut current = value;
    for key in field_path.split('.') {
        current = current.get(key)?;
    }
    Some(current)
}

fn collect_metrics_from_single_json(
    metrics_path: &str,
    filters: &Map<String, Value>,
) -> anyhow::Result<String> {
    // Open the specified JSON file and parse
    let file = File::open(metrics_path)
        .with_context(|| format!("Failed to open metrics file '{}'", metrics_path))?;
    let reader = BufReader::new(file);
    let data: Value = serde_json::from_reader(reader)
        .with_context(|| format!("Failed to parse metrics file '{}' as JSON", metrics_path))?;

    let mut metrics = String::with_capacity(128); // Adjust based on expected size

    // Apply each filter
    for (field_path, rename) in filters.iter() {
        let field_value = get_nested_value(&data, field_path);
        if field_value.is_none() {
            return Err(anyhow!(
                "Required field '{}' not found in '{}' (from metrics config)",
                field_path,
                metrics_path
            ));
        }

        // If present, append to metrics
        if let Some(val) = field_value {
            // Convert field_value to string
            let val_str = match val {
                Value::String(s) => s.to_string(),
                Value::Number(num) => num.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => {
                    // For complex structures, we can serialize the entire value
                    serde_json::to_string(val).with_context(|| {
                        format!(
                            "Failed to serialize field '{}' in JSON '{}'",
                            field_path, metrics_path
                        )
                    })?
                }
            };

            let mut rename_str = field_path;
            if let Value::String(s) = rename {
                if !s.is_empty() {
                    rename_str = s;
                }
            }

            if !metrics.is_empty() {
                write!(&mut metrics, ",").with_context(|| {
                    format!(
                        "Failed to append ',' to metrics string for field '{}' in JSON '{}'",
                        field_path, metrics_path
                    )
                })?;
            }
            write!(&mut metrics, "{}={}", rename_str, val_str.trim_matches('%')).with_context(
                || {
                    format!(
                        "Failed to append '{}={}' to metrics string for JSON '{}'",
                        rename_str,
                        val_str.trim_matches('%'),
                        metrics_path
                    )
                },
            )?;
        }
    }

    println!("sub metrics: {}", metrics);

    Ok(metrics)
}

/// reference: .github/templates/metrics_template.json
fn collect_metrics(metrics_config: &str) -> anyhow::Result<String> {
    let file = File::open(metrics_config)
        .with_context(|| format!("Failed to open metrics config file '{}'", metrics_config))?;
    let reader = BufReader::new(file);
    let configs: Value = serde_json::from_reader(reader).with_context(|| {
        format!(
            "Failed to parse metrics config file '{}' as JSON",
            metrics_config
        )
    })?;

    let pairs = configs
        .as_array()
        .ok_or_else(|| anyhow!("Config JSON must be an array of path/filter pairs."))?;

    let mut metrics = String::with_capacity(1024); // Adjust based on expected size

    for pair in pairs {
        let path = pair
            .get("path")
            .and_then(|p| p.as_str())
            .ok_or_else(|| anyhow!("Each config pair must have a 'path' field"))?;

        let filters = pair
            .get("filters")
            .and_then(|f| f.as_object())
            .ok_or_else(|| anyhow!("Each config pair must have a 'filters' object"))?;

        let sub_metrics = collect_metrics_from_single_json(path, filters)?;

        if !sub_metrics.is_empty() {
            if !metrics.is_empty() {
                metrics.push(',');
            }
            metrics.push_str(&sub_metrics);
        }
    }

    println!("metrics: {}", metrics);

    Ok(metrics)
}

pub fn update_db(metrics_config: &str, sub_crate: &str, bench: &str) -> anyhow::Result<()> {
    let influxdb_url = env::var("INFLUXDB_URL").with_context(|| "INFLUXDB_URL not set")?;
    let influxdb_token = env::var("INFLUXDB_TOKEN").with_context(|| "INFLUXDB_TOKEN not set")?;
    let influxdb_org = env::var("INFLUXDB_ORG").with_context(|| "INFLUXDB_ORG not set")?;
    let influxdb_bucket = env::var("INFLUXDB_BUCKET").with_context(|| "INFLUXDB_BUCKET not set")?;

    let metrics = collect_metrics(metrics_config)?;
    if metrics.is_empty() {
        return Ok(());
    }

    let timestamp = chrono::Utc::now()
        .timestamp_nanos_opt()
        .ok_or(anyhow!("Failed to get current timestamp"))?;

    const SINGLE_SPACE: char = ' ';
    // Prepare the InfluxDB line protocol data
    // Include all relevant statistics
    let line = format!(
        "{MEASUREMENT_NAME},qname={sub_crate},impl={}{SINGLE_SPACE}{metrics}{SINGLE_SPACE}{timestamp}",
        bench.trim_start_matches("bench_"),
    );

    // Send the data to InfluxDB
    let url = format!(
        "{}/api/v2/write?org={}&bucket={}&precision=ns",
        influxdb_url, influxdb_org, influxdb_bucket
    );

    let client = Client::new();

    let response = client
        .post(url)
        .header("Authorization", format!("Token {}", influxdb_token))
        .header("Content-Type", "text/plain; charset=utf-8")
        .body(line)
        .send()?;

    if response.status().is_success() {
        println!("Successfully wrote data for {sub_crate} {bench}",);
    } else {
        eprintln!(
            "Failed to write data for {sub_crate} {bench}. Status: {}. Body: {}",
            response.status(),
            response.text()?
        );
    }

    Ok(())
}
