use failure::Error;
use fehler::throws;
use kraken_futures::rest::{KrakenRest, OrderbookRequest, TickersRequest};
use tokio::runtime::Runtime;

#[test]
#[throws(Error)]
fn test_orderbook() {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();

    let mut rt = Runtime::new()?;

    let client = KrakenRest::new(None);
    rt.block_on(client.request(OrderbookRequest { symbol: "PI_XBTUSD".parse()? }))?;
}

#[test]
#[throws(Error)]
fn test_tickers() {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();

    let mut rt = Runtime::new()?;

    let client = KrakenRest::new(None);

    rt.block_on(client.request(TickersRequest))?;
}
