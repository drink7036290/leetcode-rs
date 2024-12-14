use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// A JSON configuration file describing what metrics to gather.
    #[arg(long, value_name = "METRICS_CONFIG_PATH")]
    pub metrics_config: String,

    /// e.g., "qxxx_with_blabla"
    #[arg(long, value_name = "SUB_CRATE_NAME")]
    pub sub_crate: String,

    /// e.g., "bench_IMPL"
    #[arg(long, value_name = "BENCH_NAME")]
    pub bench: String,
}
