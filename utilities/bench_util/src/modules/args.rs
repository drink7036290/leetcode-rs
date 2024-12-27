use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "bench_util")]
pub enum Command {
    /// Update the database with new metrics.
    UpdateDb {
        /// A JSON configuration file describing what metrics to gather.
        #[arg(long, value_name = "METRICS_CONFIG_PATH")]
        metrics_config: String,

        /// e.g., `qxxx_with_blabla`
        #[arg(long, value_name = "SUB_CRATE_NAME")]
        sub_crate: String,

        /// e.g., `bench_IMPL`
        #[arg(long, value_name = "BENCH_NAME")]
        bench: String,
    },
    /// Update the dashboard's time range.
    UpdateDashboardTimeRange,
}

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}
