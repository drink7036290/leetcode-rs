mod modules;
use modules::dashboard;
use modules::db;

fn main() -> anyhow::Result<()> {
    match db::update_db()? {
        db::DBStatus::Updated => dashboard::update_dashboard_time_range()?,
        db::DBStatus::NoUpdate => println!("No new data to update the Grafana dashboard"),
    }

    Ok(())
}
