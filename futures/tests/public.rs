use failure::Error;
use fehler::throws;
use kraken_futures::rest::{AccountsRequest, Kraken, OrderbookRequest, TickersRequest};
use structopt::StructOpt;
use tokio::runtime::Runtime;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "HoneyDeer", about = "The HoneyDeer.")]
struct Opt {
    #[structopt(env)]
    kraken_api_key: String,
    #[structopt(env)]
    kraken_api_secret: String,
}

#[test]
#[throws(Error)]
fn test_orderbook() {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();

    let mut rt = Runtime::new()?;

    let client = Kraken::new();
    rt.block_on(client.request(OrderbookRequest { symbol: "PI_XBTUSD".parse()? }))?;
}

#[test]
#[throws(Error)]
fn test_account() {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();

    let opt = Opt::from_args();

    let mut rt = Runtime::new()?;

    let client = Kraken::with_credential(&opt.kraken_api_key, &opt.kraken_api_secret);

    rt.block_on(client.request(AccountsRequest))?;
}

#[test]
#[throws(Error)]
fn test_tickers() {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();

    let mut rt = Runtime::new()?;

    let client = Kraken::new();

    rt.block_on(client.request(TickersRequest))?;
}
