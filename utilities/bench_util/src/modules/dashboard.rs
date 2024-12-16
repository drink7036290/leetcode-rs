use super::constants::*;
use anyhow::{anyhow, Context, Ok, Result};
use chrono::{DateTime, Duration, Utc};
use csv::{ReaderBuilder, StringRecord};
use reqwest::blocking::Client;
use retry::{delay::Exponential, retry, OperationResult};
use serde_json::Value;
use std::env;
use std::io::Cursor;

pub fn update_dashboard_time_range() -> Result<()> {
    let (grafana_url, grafana_token, dashboard_uid) = load_grafana_env()?;

    let client = Client::new();

    let mut dashboard_json: Value =
        fetch_dashboard(&client, &grafana_url, &grafana_token, &dashboard_uid)?;

    if update_time_range(&mut dashboard_json)?.is_none() {
        return Ok(());
    }

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
fn update_time_range(dashboard_json: &mut Value) -> Result<Option<()>> {
    // Access the "dashboard" field
    let dashboard = dashboard_json
        .get_mut("dashboard")
        .ok_or_else(|| anyhow!("Missing 'dashboard' field"))?;

    let (mut time_start, mut time_end) = match get_time_range(dashboard)? {
        Some(t) => t,
        None => return Ok(None),
    };

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

    Ok(Some(()))
}

fn get_time_range(dashboard: &Value) -> Result<Option<(DateTime<Utc>, DateTime<Utc>)>> {
    // Access the "templating" object
    let templating = dashboard
        .get("templating")
        .and_then(|v| v.as_object())
        .ok_or_else(|| anyhow!("Missing or invalid 'templating'"))?;

    // Access the "list" array
    let list = templating
        .get("list")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow!("'list' is not an array"))?;

    // get COMMIT_RANGE from the list
    let var = list
        .iter()
        .find(|v| {
            v.get("name")
                .and_then(|n| n.as_str())
                .map_or_else(|| false, |n| n == COMMIT_RANGE)
        })
        .ok_or_else(|| anyhow!("No '{}' variable", COMMIT_RANGE))?;

    let query_val = var
        .get("query")
        .ok_or_else(|| anyhow!("No 'query' field"))?;

    match query_val {
        Value::String(s) => get_first_and_last_commit_from_db(s),
        Value::Object(o) => {
            let query_str = o
                .get("query")
                .and_then(|q| q.as_str())
                .ok_or_else(|| anyhow!("No 'query' field in object"))?;

            get_first_and_last_commit_from_db(query_str)
        }
        _ => Err(anyhow!("Unexpected 'query' field type")),
    }
}

fn get_first_and_last_commit_from_db(
    query: &str,
) -> Result<Option<(DateTime<Utc>, DateTime<Utc>)>> {
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

    // seems like the api/v2/query only supports csv output format for now
    let text = response.text()?;
    println!("influxdb response:\n{}\n", text);
    let mut rdr = ReaderBuilder::new()
        .has_headers(true) // set to true if the first line is headers
        .from_reader(Cursor::new(text));

    // posibility no headers ?
    let headers = rdr.headers()?.clone(); // Clone headers for repeated lookup

    let mut records_iter = rdr.records();
    let first_record = match records_iter.next() {
        Some(r) => r?,
        None => return Ok(None),
    };

    // Check if there's another record
    if records_iter.next().is_some() {
        return Err(anyhow!("More than one record found"));
    }

    // Process the single record
    let first_commit = get_csv_column_value(&headers, &first_record, "first_commit")?;
    let last_commit = get_csv_column_value(&headers, &first_record, "last_commit")?;
    Ok(Some((
        chrono::DateTime::parse_from_rfc3339(first_commit)?.with_timezone(&chrono::Utc),
        chrono::DateTime::parse_from_rfc3339(last_commit)?.with_timezone(&chrono::Utc),
    )))
}

fn get_csv_column_value<'a>(
    headers: &StringRecord,
    record: &'a StringRecord,
    column_name: &str,
) -> Result<&'a str> {
    let idx = headers
        .iter()
        .position(|h| h == column_name)
        .ok_or_else(|| anyhow!("No '{}' column", column_name))?;

    record
        .get(idx)
        .ok_or_else(|| anyhow!("No '{}' value", column_name))
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
