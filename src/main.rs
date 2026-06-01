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

    let token = client.fetch_access_token().await?;

    println!("token: {}", token);

    //
    let lamin: f64 = 0.0;
    let lamax: f64 = 90.0;

    let state = client.get_state(lamin, lamax).await?;

    println!("{:?}", state);

    Ok(())
}
