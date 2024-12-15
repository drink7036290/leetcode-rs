use super::constants::*;
use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Duration, Utc};
use reqwest::blocking::Client;
use retry::{delay::Exponential, retry, OperationResult};
use serde_json::Value;
use std::env;

pub fn update_dashboard_time_range() -> Result<()> {
    let (grafana_url, grafana_token, dashboard_uid) = load_grafana_env()?;

    let client = Client::new();

    let mut dashboard_json: Value =
        fetch_dashboard(&client, &grafana_url, &grafana_token, &dashboard_uid)?;

    update_time_range(&mut dashboard_json)?;

    update_dashboard(&client, &grafana_url, &grafana_token, &mut dashboard_json)
}

fn load_grafana_env() -> Result<(String, String, String)> {
    // Load environment variables
    let grafana_url = env::var("GRAFANA_URL").context("GRAFANA_URL not set")?;
    let grafana_token = env::var("GRAFANA_SERVICE_ACCOUNT_TOKEN")
        .context("GRAFANA_SERVICE_ACCOUNT_TOKEN not set")?;
    let dashboard_uid =
        env::var("GRAFANA_DASHBOARD_UID").context("GRAFANA_DASHBOARD_UID not set")?;

    Ok((grafana_url, grafana_token, dashboard_uid))
}

fn fetch_dashboard(
    client: &Client,
    grafana_url: &str,
    grafana_token: &str,
    dashboard_uid: &str,
) -> Result<Value> {
    let operation = || {
        let response = client
            .get(format!(
                "{}/api/dashboards/uid/{}",
                grafana_url, dashboard_uid
            ))
            .bearer_auth(grafana_token)
            .send()
            .with_context(|| "Failed to send request to Grafana API")?;

        let status = response.status();
        match status.as_u16() {
            200..=299 => Ok(OperationResult::Ok(response)), // is_success()
            500..=599 => Ok(OperationResult::Retry(anyhow!("Server error: {}", status))), // is_server_error()
            429 => Ok(OperationResult::Retry(anyhow!("Rate limited: {}", status))),
            400..=499 => Err(anyhow!(
                "Failed to fetch dashboard: Client error: {} - {}",
                status,
                response
                    .text()
                    .unwrap_or_else(|_| "Unknown error".to_string())
            )),
            _ => Err(anyhow!(
                "Failed to fetch dashboard: Unhandled status: {} - {}",
                status,
                response
                    .text()
                    .unwrap_or_else(|_| "Unknown error".to_string())
            )),
        }
    };

    let retry_strategy = Exponential::from_millis(INITIAL_DELAY_MS).take(MAX_RETRIES);

    let response = match retry(retry_strategy, operation).map_err(|e| anyhow!(e))? {
        OperationResult::Ok(response) => response,
        OperationResult::Retry(err) | OperationResult::Err(err) => return Err(err),
    };

    response
        .json()
        .with_context(|| "Failed to parse dashboard JSON")
}

// reference: Grafana's Dashboard JSON Model
fn update_time_range(dashboard_json: &mut Value) -> Result<()> {
    // Access the "dashboard" field
    let dashboard = dashboard_json
        .get_mut("dashboard")
        .ok_or_else(|| anyhow!("Missing 'dashboard' field"))?;

    // Access the "templating" object
    let templating = dashboard
        .get_mut("templating")
        .and_then(|v| v.as_object_mut())
        .ok_or_else(|| anyhow!("Missing or invalid 'templating'"))?;

    // Access the "list" array
    let list = templating
        .get_mut("list")
        .and_then(|v| v.as_array_mut())
        .ok_or_else(|| anyhow!("'list' is not an array"))?;

    let mut time_end = Utc::now();
    let mut time_start = time_end - Duration::days(1); // default time

    // Iterate through the array of variables
    for var in list.iter_mut() {
        // Check if name is "_earliest_commit"
        if let Some(name) = var.get("name").and_then(|n| n.as_str()) {
            if name == "_earliest_commit" {
                // Access the query field if present
                if let Some(query_val) = var.get("query") {
                    if let Some(query_str) = query_val.as_str() {
                        time_start = query_time_from_db(query_str)?;
                    }
                }
            }
        }
    }

    // Expand the time range by 10% on each side
    let shift = (time_end - time_start).num_seconds() / 10;
    time_start -= Duration::seconds(shift);
    time_end += Duration::seconds(shift);

    println!(
        "Setting Dashboard Time Range from {} to {}",
        time_start, time_end
    );

    dashboard["time"]["from"] = Value::String(time_start.to_rfc3339());
    dashboard["time"]["to"] = Value::String(time_end.to_rfc3339());

    Ok(())
}

fn query_time_from_db(query: &str) -> Result<DateTime<Utc>> {
    // Fetch environment variables
    let influxdb_url = env::var("INFLUXDB_URL").context("INFLUXDB_URL not set")?;
    let influxdb_token = env::var("INFLUXDB_TOKEN").context("INFLUXDB_TOKEN not set")?;
    let influxdb_org = env::var("INFLUXDB_ORG").context("INFLUXDB_ORG not set")?;

    let client = Client::new();

    // Replace placeholders
    let new_query = query
        .replace("${InfluxDB_Bucket}", INFLUXDB_BUCKET)
        .replace("${Measurement}", MEASUREMENT_NAME);

    let response = client
        .post(format!("{}/api/v2/query", influxdb_url))
        .header("Authorization", format!("Token {}", influxdb_token))
        .header("Content-Type", "application/vnd.flux")
        .query(&[("org", influxdb_org)])
        .body(new_query)
        .send()?;

    let text = response.text()?;
    let mut time_start = Utc::now() - Duration::days(1); // default time

    // Parse CSV response and extract times
    let lines: Vec<&str> = text.lines().take(2).collect();
    if lines.len() < 2 {
        // maybe influxdb has flushed all the data, return a default time
        return Ok(time_start);
    }

    // Example response:
    // ",result,table,text"
    // ",_result,0,2024-12-05T01:42:34.008261000Z"

    let titles = lines[0].split(',');
    let values = lines[1].split(',');

    for (title, value) in titles.zip(values) {
        if title == "text" {
            time_start = chrono::DateTime::parse_from_rfc3339(value)?.with_timezone(&chrono::Utc);
            break;
        }
    }

    Ok(time_start)
}

fn update_dashboard(
    client: &Client,
    grafana_url: &str,
    grafana_token: &str,
    dashboard_json: &mut Value,
) -> Result<()> {
    // Prepare the payload
    let payload = serde_json::json!({
        "dashboard": dashboard_json["dashboard"],
        "overwrite": true
    });

    // Update the dashboard
    let response = client
        .post(format!("{}/api/dashboards/db", grafana_url))
        .bearer_auth(grafana_token)
        .json(&payload)
        .send()
        .with_context(|| "Failed to send update request to Grafana API")?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(anyhow!(
            "Failed to update dashboard: {} - {}",
            status,
            error_text
        ));
    }

    println!("Dashboard time range updated successfully.");

    Ok(())
}
