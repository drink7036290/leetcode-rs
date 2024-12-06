use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Duration, Utc};
use dotenvy::dotenv;
use reqwest::blocking::Client;
use serde_json::Value;
use std::env;

fn get_time_start() -> Result<DateTime<Utc>> {
    // Fetch environment variables
    let influxdb_url = env::var("INFLUXDB_URL").context("INFLUXDB_URL not set")?;
    let influxdb_token = env::var("INFLUXDB_TOKEN").context("INFLUXDB_TOKEN not set")?;
    let influxdb_org = env::var("INFLUXDB_ORG").context("INFLUXDB_ORG not set")?;

    let client = Client::new();

    let query = r#"
    // Flux query to get earliest data saved in InfluxDB instead of GitHub's earliest commit
    // Adjust the query as needed
    from(bucket: "leetcode-rs_bench")
        |> range(start: -30d)
        |> filter(fn: (r) =>
          r._measurement == "benchmark" and
          exists r.name and
          r._field == "slope_point_estimate"
        )

        // split into smaller groups(by series) 
        |> group()

        // sort each group based on _time; collect the smallest one from each group;
        // sort them again as group level and get the smallest one
        |> first() 
        
        |> map(fn: (r) => ({
            text: string(v: r._time)
        }))
    "#;

    let response = client
        .post(format!("{}/api/v2/query", influxdb_url))
        .header("Authorization", format!("Token {}", influxdb_token))
        .header("Content-Type", "application/vnd.flux")
        .query(&[("org", influxdb_org)])
        .body(query)
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

fn update_time_range(
    dashboard_json: &mut Value,
    time_start: DateTime<Utc>,
    time_end: DateTime<Utc>,
) -> Result<()> {
    let dashboard = dashboard_json
        .get_mut("dashboard")
        .ok_or_else(|| anyhow!("Missing 'dashboard' field"))?;

    dashboard["time"]["from"] = Value::String(time_start.to_rfc3339());
    dashboard["time"]["to"] = Value::String(time_end.to_rfc3339());

    Ok(())
}

fn update_dashboard(time_start: DateTime<Utc>, time_end: DateTime<Utc>) -> Result<()> {
    // Load environment variables
    let grafana_url = env::var("GRAFANA_URL").context("GRAFANA_URL not set")?;
    let grafana_token = env::var("GRAFANA_SERVICE_ACCOUNT_TOKEN")
        .context("GRAFANA_SERVICE_ACCOUNT_TOKEN not set")?;
    let dashboard_uid =
        env::var("GRAFANA_DASHBOARD_UID").context("GRAFANA_DASHBOARD_UID not set")?;

    // Fetch the current dashboard JSON
    let client = Client::new();
    let response = client
        .get(format!(
            "{}/api/dashboards/uid/{}",
            grafana_url, dashboard_uid
        ))
        .bearer_auth(&grafana_token)
        .send()
        .with_context(|| "Failed to send request to Grafana API")?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(anyhow!(
            "Failed to fetch dashboard: {} - {}",
            status,
            error_text
        ));
    }

    let mut dashboard_json: Value = response
        .json()
        .with_context(|| "Failed to parse dashboard JSON")?;

    // Update the time range
    update_time_range(&mut dashboard_json, time_start, time_end)?;

    // Prepare the payload
    let payload = serde_json::json!({
        "dashboard": dashboard_json["dashboard"],
        "overwrite": true
    });

    // Update the dashboard
    let response = client
        .post(format!("{}/api/dashboards/db", grafana_url))
        .bearer_auth(&grafana_token)
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

pub fn update_dashboard_time_range() -> Result<()> {
    // Load environment variables from .env file (optional)
    dotenv().ok();

    // Calculate desired time range
    let mut time_end = Utc::now();
    let mut time_start = get_time_start()?;

    // Expand the time range by 10% on each side
    let shift = (time_end - time_start).num_seconds() / 10;
    time_start -= Duration::seconds(shift);
    time_end += Duration::seconds(shift);

    println!(
        "Setting Dashboard Time Range from {} to {}",
        time_start, time_end
    );

    // Update dashboard
    update_dashboard(time_start, time_end)?;

    Ok(())
}
