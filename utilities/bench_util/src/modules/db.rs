use anyhow::{anyhow, Ok};
use clap::Parser;
use dotenvy::dotenv;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::{env, path::PathBuf};
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

#[derive(Parser, Debug)]
struct Args {
    /// Directory path for benchmark metrics like maximum RSS and CPU usage
    #[arg(long, value_name = "DIR", requires_all = &["sub_crate_name", "bench_name"])]
    bench_metrics_dir: Option<String>,

    /// Sub-crate name, e.g., "qxxx"
    #[arg(long, value_name = "SUB_CRATE", requires_all = &["bench_metrics_dir", "bench_name"])]
    sub_crate_name: Option<String>,

    /// Benchmark name, e.g., "bench_IMPL"
    #[arg(long, value_name = "BENCH_NAME", requires_all = &["bench_metrics_dir", "sub_crate_name"])]
    bench_name: Option<String>,
}

fn move_criterion_dir(
    criterion_dir: &mut PathBuf,
    sub_crate_name: &str,
    bench_name: &str,
) -> anyhow::Result<()> {
    // e.g., qxxx_with_blabla
    let sub_crate_prefix = sub_crate_name.split('_').next().ok_or_else(|| {
        anyhow!(
            "Sub-crate name \"{}\" must be in the format of \"qxxx_with_blabla\"",
            sub_crate_name
        )
    })?;

    println!("before criterion_dir: {}", criterion_dir.display());

    // Extend criterion_dir to this specific benchmark
    criterion_dir.push(format!(
        "{}_with_{}", // e.g., "qxxx_IMPL"
        sub_crate_prefix,
        bench_name.trim_start_matches("bench_")
    ));

    if criterion_dir.try_exists().is_err() {
        return Err(anyhow!(
            "Criterion directory \"{}\" does not exist",
            criterion_dir.display()
        ));
    }

    println!("criterion_dir: {}", criterion_dir.display());

    Ok(())
}

fn retrieve_metrics_info(criterion_dir: &mut PathBuf) -> anyhow::Result<String> {
    let mut metrics_info = String::new();

    let args = Args::parse();

    // e.g.,
    // bench_metrics_dir = /tmp/
    // sub_crate_name = qxxx_with_blabla
    // bench_name = bench_IMPL
    if let (Some(bench_metrics_dir), Some(sub_crate_name), Some(bench_name)) =
        (args.bench_metrics_dir, args.sub_crate_name, args.bench_name)
    {
        println!("bench_metrics_dir: {}", bench_metrics_dir);
        println!("sub_crate_name: {}", sub_crate_name);
        println!("bench_name: {}", bench_name);

        move_criterion_dir(criterion_dir, &sub_crate_name, &bench_name)?;

        let bench_metrics_dir = PathBuf::from(bench_metrics_dir);
        if bench_metrics_dir.try_exists().is_err() {
            return Err(anyhow!(
                "Benchmark metrics directory \"{}\" does not exist",
                bench_metrics_dir.display()
            ));
        }

        // for each bench_metrics_dir/FILE, append ",FILE_NAME=FILE_CONTENT" to metrics_info
        for entry in WalkDir::new(bench_metrics_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_string_lossy();
            let file_content = fs::read_to_string(path)?;
            metrics_info.push_str(&format!(
                ",{}={}",
                file_name,
                file_content.trim().trim_matches('%')
            ));
        }
    } else {
        println!("Running without benchmark metrics.");
    }

    println!("metrics_info: {}", metrics_info);

    Ok(metrics_info)
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
    let mut criterion_dir = workspace_root.join("target").join("criterion");
    let mut db_updated = false;
    let metrics_info = retrieve_metrics_info(&mut criterion_dir)?;

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
            slope_lower_bound={},slope_upper_bound={},slope_point_estimate={}{} {}",
            benchmark_name, // name
            impl_name,      // impl
            estimates.slope.confidence_interval.lower_bound,
            estimates.slope.confidence_interval.upper_bound,
            estimates.slope.point_estimate,
            metrics_info,
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
