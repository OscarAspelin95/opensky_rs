use std::{io::Read, path::Path};

use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
}

impl Credentials {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
        }
    }
}

impl TryFrom<&Path> for Credentials {
    type Error = anyhow::Error;

    fn try_from(fpath: &Path) -> Result<Self, Self::Error> {
        let mut f = File::open(fpath)?;

        let mut buf: Vec<u8> = vec![];
        let _ = f.read_to_end(&mut buf)?;

        let credentials: Self = serde_json::from_reader(&buf[..])?;

        Ok(credentials)
    }
}
