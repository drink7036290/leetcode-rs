mod modules;
use clap::Parser;
use dotenvy::dotenv;
use modules::args::Args;
use modules::dashboard;
use modules::db;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Load environment variables
    dotenv().ok(); // consuming the error if no .env file

    match db::update_db(&args)? {
        db::DBStatus::Updated => dashboard::update_dashboard_time_range()?,
        db::DBStatus::NoUpdate => println!("No new data to update the Grafana dashboard"),
    }

    Ok(())
}
