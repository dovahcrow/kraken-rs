#![allow(unused_imports)]

use dotenv::dotenv;
use env_logger::init;
use failure::Error;
use fehler::throws;
use futures::{SinkExt, StreamExt};
use kraken_futures::rest::{KrakenRest, TickersRequest};
use kraken_futures::ws::{message, Command, KrakenWebsocket};
use kraken_futures::{Side, Symbol};
use serde_json::from_str;
use std::collections::BTreeMap;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "kraken-rs", about = "kraken-rs.")]
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

    let mut ws = KrakenWebsocket::with_credential(&opt.kraken_api_key, &opt.kraken_api_secret).await?;

    ws.send(Command::Ping).await?;

    while let Some(Ok(e)) = ws.next().await {
        match e {
            message::Message::Subscription(_) => unreachable!(),
            message::Message::Info { version, .. } => println!("Kraken Version {}", version),
            message::Message::Subscribed { feed, extra, .. } => println!("Subscribed to {}: {:?}", feed, extra),
            message::Message::Pong => println!("Pong"),
            m => println!("Unknown {:?}", m),
        }
    }

    Ok(())
}
