use anyhow::Error;
use dotenv::dotenv;
use env_logger::init;
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

    let client = KrakenRest::with_credential(None, &opt.kraken_api_key, &"aa");
    let resp = client.request(AccountsRequest).await?;
    println!("{:?}", resp);
    Ok(())
}
