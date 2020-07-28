#![allow(unused_imports)]

use dotenv::dotenv;
use env_logger::init;
use failure::Error;
use fehler::throws;
use futures::{SinkExt, StreamExt};
use kraken_futures::rest::{Kraken, TickersRequest};
use kraken_futures::ws::{message, Command, KrakenWebsocket};
use kraken_futures::Symbol;
use serde_json::from_str;
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

    // let client = Kraken::with_credential(&opt.kraken_api_key, &opt.kraken_api_secret);
    // let _ = client.request(OrderbookRequest { symbol: "PI_XBTUSD".into() }).await?;

    // let resp = client.request(TickersRequest).await?;
    // let sym: Symbol = "fi_xbtusd".parse()?;
    // println!("aaa {:?}", sym);

    let mut ws = KrakenWebsocket::with_credential(&opt.kraken_api_key, &opt.kraken_api_secret).await?;

    ws.send(Command::book(&["PI_XBTUSD"])?).await?;
    ws.send(Command::challenge(&opt.kraken_api_key)).await?;
    let mut challenge = None;

    while let Some(Ok(e)) = ws.next().await {
        match e {
            message::Message::Subscription(_) => {}
            message::Message::Info { .. } => {}
            message::Message::Subscribed { feed, .. } => println!("Subscribed to {}", feed),
            message::Message::Challenge { message, .. } => {
                println!("Challenge received {}", message);
                challenge = Some(message);
                break;
            }
            _ => unreachable!(),
        }
    }

    ws.send((Command::fills(), &challenge.unwrap())).await?;

    while let Some(Ok(e)) = ws.next().await {
        match e {
            message::Message::Subscription(s) => match s {
                message::SubscriptionMessage::BookSnapshot(b) => println!("Subscription {:?}", b),
                message::SubscriptionMessage::Book(b) => println!("Subscription {:?}", b),
                message::SubscriptionMessage::FillsSnapshot(b) => println!("Subscription {:?}", b),
                _ => {}
            },
            message::Message::Info { .. } => {}
            message::Message::Subscribed { feed, .. } => println!("Subscribed to {}", feed),
            message::Message::Challenge { .. } => {}
            _ => unreachable!(),
        }
    }
    Ok(())
}
