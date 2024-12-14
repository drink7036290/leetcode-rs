mod modules;
use clap::Parser;
use dotenvy::dotenv;
use modules::args::{Args, Command};
use modules::dashboard;
use modules::db;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Load environment variables
    dotenv().ok(); // consuming the error if no .env file

    match args.command {
        Command::UpdateDb {
            metrics_config,
            sub_crate,
            bench,
        } => db::update_db(&metrics_config, &sub_crate, &bench),
        Command::UpdateDashboardTimeRange => dashboard::update_dashboard_time_range(),
    }
}
