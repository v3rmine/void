use warframe_market::prelude::*;

#[tokio::main]
async fn main() -> Result<(), warframe_market::Error> {
    tracing_subscriber::fmt::fmt().pretty().init();

    let client = WarframeMarket::default();

    Ok(())
}
