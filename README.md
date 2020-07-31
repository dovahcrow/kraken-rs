Kraken API Client for Rust Language
=================

### Implementation status:

Currently this library only implemented a part of the API for **Kraken Futures** (rest and websocket). But the plan is to also implement spot market API in this library.

### Usage

#### Restful

All the requests/responses are typed into structs/enums. Pass the request object to `KrakenRest::request` for calling the API.

Calling a public API:

```rust
// futures/examples/tickers.rs

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
```

Calling a private API:

```rust
// futures/examples/account.rs

use dotenv::dotenv;
use env_logger::init;
use failure::Error;
use kraken_futures::rest::{AccountsRequest, KrakenRest};
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

    // None means using the default server url. You can fill in Kraken's internal URL here if you whitelisted your IP.
    let client = KrakenRest::with_credential(None, &opt.kraken_api_key, &opt.kraken_api_secret); 
    let resp = client.request(AccountsRequest).await?;
    println!("{:?}", resp);
    Ok(())
}
```

Get your account information using rest API call:

```rust
// futures/examples/account.rs

use dotenv::dotenv;
use env_logger::init;
use failure::Error;
use kraken_futures::rest::{AccountsRequest, KrakenRest};
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

    let client = KrakenRest::with_credential(None, &opt.kraken_api_key, &opt.kraken_api_secret);
    let resp = client.request(AccountsRequest).await?;
    println!("{:?}", resp);
    Ok(())
}
```

#### Websocket

Websocket examples are quite long. Please take a look at the files in examples folder for reference.

* [Subscribe to book](futures/examples/book.rs)
* [Subscribe to fills](futures/examples/fills.rs)

#### More examples

More examples are located in the [examples](futures/examples) folder and the [tests](futures/tests) folder.