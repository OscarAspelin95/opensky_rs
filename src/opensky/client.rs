use crate::opensky::consts::OPENSKY_ROOT_URL;
use std::{io::Read, path::Path};

use crate::opensky::credentials::Credentials;
use anyhow::Result;
use std::fs::File;

#[derive(Debug)]
pub struct Client {
    credentials: Credentials,
    url: &'static str,
}

impl Client {
    pub fn try_from_json(credentials_json: &Path) -> Result<Self> {
        let mut f = File::open(credentials_json)?;

        let mut buf: Vec<u8> = vec![];
        let _ = f.read_to_end(&mut buf)?;

        let credentials: Credentials = serde_json::from_reader(&buf[..])?;

        Ok(Self {
            credentials,
            url: OPENSKY_ROOT_URL,
        })
    }

    pub fn from_credentials(client_id: String, client_secret: String) -> Self {
        let credentials = Credentials::new(client_id, client_secret);

        Self {
            credentials,
            url: OPENSKY_ROOT_URL,
        }
    }
}
