use dotenvy::dotenv;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::BufReader;
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
struct Estimates {
    mean: Statistic,
    median: Statistic,
    median_abs_dev: Statistic,
    slope: Statistic,
    std_dev: Statistic,
    // Add other fields if present in the JSON
}

#[derive(Debug, Deserialize)]
struct Statistic {
    confidence_interval: ConfidenceInterval,
    point_estimate: f64,
    standard_error: f64,
}

#[derive(Debug, Deserialize)]
struct ConfidenceInterval {
    confidence_level: f64,
    lower_bound: f64,
    upper_bound: f64,
}

//fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenv()?;

    // Fetch environment variables
    let influxdb_url = env::var("INFLUXDB_URL")?;
    let influxdb_token = env::var("INFLUXDB_TOKEN")?;
    let influxdb_org = env::var("INFLUXDB_ORG")?;
    let influxdb_bucket = env::var("INFLUXDB_BUCKET")?;
    let git_commit_sha = env::var("GIT_COMMIT_SHA").unwrap_or_else(|_| "unknown".to_string());
    let git_branch = env::var("GIT_BRANCH").unwrap_or_else(|_| "unknown".to_string());
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

    // Iterate over all estimates.json files
    for entry in WalkDir::new(&criterion_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == "estimates.json")
        // parent directory is "new"
        .filter(|e| e.path().parent().unwrap().file_name() == Some(std::ffi::OsStr::new("new")))
    {
        let path = entry.path();

        // Extract benchmark name from the directory structure
        // target/criterion/<benchmark_name>/new/estimates.json
        let parent = path.parent().unwrap(); // new/
        let benchmark_dir = parent.parent().unwrap(); // <benchmark_name>/
        let benchmark_name = benchmark_dir.file_name().unwrap().to_string_lossy();

        // Open and parse the estimates.json file
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let estimates: Estimates = serde_json::from_reader(reader)?;

        // Prepare the InfluxDB line protocol data
        // Include all relevant statistics
        let line = format!(
            "benchmark,benchmark_name=\"{}\",branch=\"{}\",commit_sha=\"{}\" \
            mean={},median={},std_dev={},slope={},median_abs_dev={},\
            mean_lb={},mean_ub={},std_dev_lb={},std_dev_ub={},\
            slope_lb={},slope_ub={},median_abs_dev_lb={},median_abs_dev_ub={} {}",
            benchmark_name,
            git_branch,
            git_commit_sha,
            estimates.mean.point_estimate,
            estimates.median.point_estimate,
            estimates.std_dev.point_estimate,
            estimates.slope.point_estimate,
            estimates.median_abs_dev.point_estimate,
            estimates.mean.confidence_interval.lower_bound,
            estimates.mean.confidence_interval.upper_bound,
            estimates.std_dev.confidence_interval.lower_bound,
            estimates.std_dev.confidence_interval.upper_bound,
            estimates.slope.confidence_interval.lower_bound,
            estimates.slope.confidence_interval.upper_bound,
            estimates.median_abs_dev.confidence_interval.lower_bound,
            estimates.median_abs_dev.confidence_interval.upper_bound,
            timestamp,
        );

        //let _ = estimates.mean.standard_error;

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
            println!("Successfully wrote data for benchmark: {}", benchmark_name);
        } else {
            eprintln!(
                "Failed to write data for benchmark: {}. Status: {}. Body: {}",
                benchmark_name,
                response.status(),
                response.text()?
            );
        }
    }

    Ok(())
}
