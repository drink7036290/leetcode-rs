use anyhow::{anyhow, Ok};
use clap::Parser;
use dotenvy::dotenv;
use reqwest::blocking::Client;
use serde_json::{Map, Value};
use std::env;
use std::fmt::Write;
use std::fs::File;
use std::io::BufReader;

pub enum DBStatus {
    Updated,
    NoUpdate,
}

/// Example METRICS.json structure:
///
/// [
///   {
///     "path": "path/to/estimates.json",
///     "filters": {
///       "slope.confidence_interval.lower_bound": "needed",
///       "slope.confidence_interval.upper_bound": "needed",
///       "slope.point_estimate": "needed"
///     }
///   },
///   {
///     "path": "path/to/bench_metric_path.json",
///     "filters": {
///       "max_rss_in_kb": "needed",
///       "cpu_percentage": "optional",
///       "wall_clock_in_seconds": "optional"
///     }
///   }
/// ]
#[derive(Parser, Debug)]
struct Args {
    /// A JSON configuration file describing what metrics to gather.
    #[arg(long, value_name = "METRICS_CONFIG_PATH")]
    metrics_config: String,

    /// e.g., "qxxx_with_blabla"
    #[arg(long, value_name = "SUB_CRATE_NAME")]
    sub_crate: String,

    /// e.g., "bench_IMPL"
    #[arg(long, value_name = "BENCH_NAME")]
    bench: String,
}

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
    let file = File::open(metrics_path)?;
    let reader = BufReader::new(file);
    let data: Value = serde_json::from_reader(reader)?;

    let mut metrics = String::with_capacity(128); // Adjust based on expected size

    // Apply each filter
    for (field_path, requirement) in filters.iter() {
        let requirement_str = requirement.as_str().unwrap_or("optional");
        let is_needed = requirement_str.eq_ignore_ascii_case("needed");

        let field_value = get_nested_value(&data, field_path);
        if field_value.is_none() && is_needed {
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
                    serde_json::to_string(val)?
                }
            };

            if !metrics.is_empty() {
                write!(&mut metrics, ",")?;
            }
            write!(&mut metrics, "{}={}", field_path, val_str.trim_matches('%'))?;
        }
    }

    println!("sub metrics: {}", metrics);

    Ok(metrics)
}

fn collect_metrics(metrics_config: String) -> anyhow::Result<String> {
    let file = File::open(metrics_config)?;
    let reader = BufReader::new(file);
    let configs: Value = serde_json::from_reader(reader)?;

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

pub fn update_db() -> anyhow::Result<DBStatus> {
    // Load environment variables
    dotenv().ok(); // consuming the error if no .env file

    let influxdb_url = env::var("INFLUXDB_URL")?;
    let influxdb_token = env::var("INFLUXDB_TOKEN")?;
    let influxdb_org = env::var("INFLUXDB_ORG")?;
    let influxdb_bucket = env::var("INFLUXDB_BUCKET")?;

    let args = Args::parse();

    let metrics = collect_metrics(args.metrics_config)?;
    if metrics.is_empty() {
        return Ok(DBStatus::NoUpdate);
    }

    let timestamp = chrono::Utc::now()
        .timestamp_nanos_opt()
        .ok_or(anyhow!("Failed to get current timestamp"))?;

    const SINGLE_SPACE: char = ' ';
    // Prepare the InfluxDB line protocol data
    // Include all relevant statistics
    let line = format!(
        "{},qname={}{SINGLE_SPACE}{metrics}{SINGLE_SPACE}{timestamp}",
        args.bench, args.sub_crate,
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
        println!(
            "Successfully wrote data for {} {}",
            args.sub_crate, args.bench,
        );
    } else {
        eprintln!(
            "Failed to write data for {} {}. Status: {}. Body: {}",
            args.sub_crate,
            args.bench,
            response.status(),
            response.text()?
        );
    }

    Ok(DBStatus::Updated)
}
