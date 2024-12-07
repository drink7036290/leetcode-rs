use anyhow::Ok;
use dotenvy::dotenv;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::BufReader;
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
struct PartialEstimates {
    slope: PartialSlope,
}

#[derive(Debug, Deserialize)]
struct PartialSlope {
    confidence_interval: PartialConfidenceInterval,
    point_estimate: f64,
}

#[derive(Debug, Deserialize)]
struct PartialConfidenceInterval {
    lower_bound: f64,
    upper_bound: f64,
}

pub enum DBStatus {
    Updated,
    NoUpdate,
}

pub fn update_db() -> anyhow::Result<DBStatus> {
    // Load environment variables
    dotenv().ok(); // consuming the error if no .env file

    // Fetch environment variables
    let influxdb_url = env::var("INFLUXDB_URL")?;
    let influxdb_token = env::var("INFLUXDB_TOKEN")?;
    let influxdb_org = env::var("INFLUXDB_ORG")?;
    let influxdb_bucket = env::var("INFLUXDB_BUCKET")?;
    let _git_commit_sha = env::var("GIT_COMMIT_SHA").unwrap_or_else(|_| "unknown".to_string());
    let _git_branch = env::var("GIT_BRANCH").unwrap_or_else(|_| "unknown".to_string());
    let timestamp = chrono::Utc::now().timestamp_nanos_opt().unwrap();

    let client = Client::new();

    // Determine the workspace root directory
    let workspace_root = env::current_dir()?
        .ancestors()
        .next()
        .unwrap()
        .to_path_buf();

    // Build the path to the target/criterion directory
    let criterion_dir = workspace_root.join("target").join("criterion");
    let mut db_updated = false;

    // Iterate over all estimates.json files
    for entry in WalkDir::new(criterion_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == "estimates.json")
        // parent directory is "new"
        .filter(|e| e.path().parent().unwrap().file_name() == Some(std::ffi::OsStr::new("new")))
    {
        let path = entry.path();

        // Extract benchmark name from the directory structure
        // target/criterion/<benchmark_dir>/new/estimates.json
        let parent = path.parent().unwrap(); // new/
        let benchmark_dir = parent.parent().unwrap(); // <benchmark_dir>/
        let benchmark_dir_name = benchmark_dir.file_name().unwrap().to_string_lossy();
        let benchmark_info: Vec<&str> = benchmark_dir_name.split("_with_").collect::<Vec<&str>>();
        let benchmark_name = benchmark_info[0];
        let mut impl_name = benchmark_name;
        if benchmark_info.len() > 1 {
            impl_name = benchmark_info[1];
        }

        // Open and parse the estimates.json file
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let estimates: PartialEstimates = serde_json::from_reader(reader)?;

        // Prepare the InfluxDB line protocol data
        // Include all relevant statistics
        let line = format!(
            "benchmark,name={},impl={} \
            slope_lower_bound={},slope_upper_bound={},slope_point_estimate={} {}",
            benchmark_name, // name
            impl_name,      // impl
            estimates.slope.confidence_interval.lower_bound,
            estimates.slope.confidence_interval.upper_bound,
            estimates.slope.point_estimate,
            timestamp,
        );

        // Send the data to InfluxDB
        let url = format!(
            "{}/api/v2/write?org={}&bucket={}&precision=ns",
            influxdb_url, influxdb_org, influxdb_bucket
        );

        let response = client
            .post(&url)
            .header("Authorization", format!("Token {}", influxdb_token))
            .header("Content-Type", "text/plain; charset=utf-8")
            .body(line)
            .send()?;

        if response.status().is_success() {
            println!(
                "Successfully wrote data for benchmark: {} impl: {}",
                benchmark_name, impl_name
            );
        } else {
            eprintln!(
                "Failed to write data for benchmark: {}. Status: {}. Body: {}",
                benchmark_name,
                response.status(),
                response.text()?
            );
        }

        db_updated = true;
    }

    if !db_updated {
        return Ok(DBStatus::NoUpdate);
    }

    Ok(DBStatus::Updated)
}
