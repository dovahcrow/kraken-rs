#![allow(unused_imports)]

use dotenv::dotenv;
use env_logger::init;
use failure::Error;
use fehler::throws;
use futures::{SinkExt, StreamExt};
use kraken_futures::rest::{CancelAllOrderRequest, CancelOrderRequest, KrakenRest, SendOrderRequest};
use kraken_futures::ws::{message, Command, KrakenWebsocket};
use kraken_futures::{Side, Symbol};
use serde_json::from_str;
use std::collections::BTreeMap;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "HoneyDeer", about = "The HoneyDeer.")]
struct Opt {
    #[structopt(env)]
    kraken_api_key: String,
    #[structopt(env)]
    kraken_api_secret: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv()?;
    init();

    let opt = Opt::from_args();

    let client = KrakenRest::with_credential(&opt.kraken_api_key, &opt.kraken_api_secret);

    let resp = client.request(SendOrderRequest::limit(Symbol::PerpetualInverse("XBTUSD".parse()?), 1000., 1)).await?;
    println!("{:?}", resp);

    let resp = client.request(CancelAllOrderRequest::all()).await?;
    println!("{:?}", resp);

    Ok(())
}
