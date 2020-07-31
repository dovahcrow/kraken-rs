#![allow(unused_imports)]

use dotenv::dotenv;
use env_logger::init;
use fehler::throws;
use futures::{SinkExt, StreamExt};
use kraken_futures::ws::{message, Command, KrakenWebsocket};
use kraken_futures::{Side, Symbol};
use serde_json::from_str;
use std::collections::BTreeMap;
use structopt::StructOpt;
use KrakenError;

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

    let mut ws = KrakenWebsocket::with_credential(None, &opt.kraken_api_key, &opt.kraken_api_secret).await?;

    ws.send(Command::challenge()).await?;

    let mut challenge = None;
    while let Some(Ok(e)) = ws.next().await {
        match e {
            message::Message::Info { version, .. } => println!("Kraken Version {}", version),
            message::Message::Challenge { message, .. } => {
                println!("Challenge received {}", message);
                challenge = Some(message);
                break;
            }
            _ => unreachable!(),
        }
    }

    ws.send((Command::fills(), challenge.unwrap())).await?;

    while let Some(Ok(e)) = ws.next().await {
        match e {
            message::Message::Subscription(x) => match x {
                message::SubscriptionMessage::Fills { username, fills, .. } => {
                    println!("[Fill {}]: {:?}", username, fills);
                }
                message::SubscriptionMessage::FillsSnapshot { account, fills, .. } => {
                    println!("[Snapshot {}]: {:?}", account, fills);
                }
                message::SubscriptionMessage::Heartbeat { .. } => {
                    println!("Heartbeat");
                }
                _ => unreachable!(),
            },
            message::Message::Subscribed { feed, extra, .. } => println!("Subscribed to {}: {:?}", feed, extra),
            _ => unreachable!(),
        }
    }

    Ok(())
}
