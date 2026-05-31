use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long, help = "Path to opensky credentials json.")]
    pub credentials_json: PathBuf,
}
