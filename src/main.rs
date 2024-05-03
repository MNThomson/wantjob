#![allow(clippy::needless_return)]

mod db;
mod http;
mod telemetry;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if cfg!(not(debug_assertions)) {
        utils::print_banner();
    }

    telemetry::init_telemetry();

    let db = db::init_dbpool().await?;

    db::setup_database(&db).await;

    http::serve(db).await?;

    Ok(())
}
