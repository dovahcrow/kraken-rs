use fehler::throws;
use kraken_futures::{
    rest::{AccountsRequest, CancelAllOrdersRequest, CancelOrderRequest, KrakenRest, OpenPositionsRequest, SendOrderRequest},
    Symbol,
};
use structopt::StructOpt;
use tokio::runtime::Runtime;
use KrakenError;

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
fn test_account() {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();

    let opt = Opt::from_args();

    let mut rt = Runtime::new()?;

    let client = KrakenRest::with_credential(None, &opt.kraken_api_key, &opt.kraken_api_secret);

    rt.block_on(client.request(AccountsRequest))?;
}

#[test]
#[throws(Error)]
fn test_open_positions() {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();

    let opt = Opt::from_args();

    let mut rt = Runtime::new()?;

    let client = KrakenRest::with_credential(None, &opt.kraken_api_key, &opt.kraken_api_secret);

    rt.block_on(client.request(OpenPositionsRequest))?;
}

#[test]
#[throws(Error)]
fn test_buy_and_cancel() {
    let _ = dotenv::dotenv();
    let _ = env_logger::try_init();

    let opt = Opt::from_args();

    let mut rt = Runtime::new()?;

    let client = KrakenRest::with_credential(None, &opt.kraken_api_key, &opt.kraken_api_secret);

    let resp = rt.block_on(client.request(SendOrderRequest::limit(Symbol::PerpetualInverse("XBTUSD".parse()?), 30000., -1)))?;

    rt.block_on(client.request(CancelOrderRequest::from_order_id(resp.send_status.order_id().unwrap())))?;

    rt.block_on(client.request(SendOrderRequest::limit(Symbol::PerpetualInverse("XBTUSD".parse()?), 30000., -1)))?;

    rt.block_on(client.request(CancelAllOrdersRequest::all()))?;
}
