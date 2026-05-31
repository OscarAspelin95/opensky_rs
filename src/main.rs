use anyhow::Result;
use clap::Parser;

mod args;
use args::Args;

use opensky_rs::opensky::client::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let client = Client::try_from_json(&args.credentials_json)?;
    println!("{:?}", client);

    Ok(())
}
