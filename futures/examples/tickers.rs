use dotenv::dotenv;
use env_logger::init;
use failure::Error;
use kraken_futures::rest::{KrakenRest, TickersRequest};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv()?;
    init();

    let client = KrakenRest::new(None); // None means using the default server url. You can fill in Kraken's internal URL here if you whitelisted your IP.

    let resp = client.request(TickersRequest).await?;

    println!("{:?}", resp);

    Ok(())
}
