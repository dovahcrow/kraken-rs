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

    ws.send(Command::book(&["PI_XBTUSD"])?).await?;

    let mut _seq = 0;
    let mut book = Book::new();

    while let Some(Ok(e)) = ws.next().await {
        match e {
            message::Message::Subscription(x) => match x {
                message::SubscriptionMessage::BookSnapshot(message::BookSnapshot { product_id, seq, bids, asks, .. }) => {
                    assert!(matches!(product_id, Symbol::PerpetualInverse(_)));
                    assert!(seq > _seq);
                    _seq = seq;
                    book.add_multiple(Side::Buy, bids);
                    book.add_multiple(Side::Sell, asks);
                }
                message::SubscriptionMessage::Book(message::Book {
                    product_id,
                    seq,
                    price,
                    qty,
                    side,
                    ..
                }) => {
                    assert!(matches!(product_id, Symbol::PerpetualInverse(_)));
                    assert!(seq > _seq);
                    _seq = seq;
                    book.add_single(side, price, qty);
                    book.print_top();
                }
                _ => unreachable!(),
            },
            message::Message::Info { version, .. } => println!("Kraken Version {}", version),
            message::Message::Subscribed { feed, extra, .. } => println!("Subscribed to {}: {:?}", feed, extra),
            _ => unreachable!(),
        }
    }

    Ok(())
}

struct Book {
    bids: BTreeMap<i64, f64>,
    asks: BTreeMap<i64, f64>,
}

impl Book {
    fn new() -> Book {
        Book {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    fn print_top(&self) {
        let (ap, aq) = self.asks.iter().next().unwrap();
        let (bp, bq) = self.bids.iter().next_back().unwrap();

        println!("[Top] bid: {:?}, ask: {:?}", ((*ap as f64) / 100000., aq), ((*bp as f64) / 100000., bq),);
    }

    fn add_single(&mut self, side: Side, price: f64, qty: f64) {
        let iprice = (price * 100000.) as i64;

        match side {
            Side::Buy => {
                if qty == 0. {
                    self.bids.remove(&iprice);
                } else {
                    self.bids.insert(iprice, qty);
                }
            }
            Side::Sell => {
                if qty == 0. {
                    self.asks.remove(&iprice);
                } else {
                    self.asks.insert(iprice, qty);
                }
            }
        }
    }
    fn add_multiple(&mut self, side: Side, prices: Vec<message::PriceTuple>) {
        let book = match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        for message::PriceTuple { price, qty } in prices {
            let iprice = (price * 100000.) as i64;
            if qty == 0. {
                book.remove(&iprice);
            } else {
                book.insert(iprice, qty);
            }
        }
    }
}
