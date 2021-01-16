use std::process::Output;

use anyhow::{Context, Result};
use common_macros::hash_map;
use reqwest::{Client, Url};

const O_AUTH_ENDPOINT: &str = "https://accounts.spotify.com/authorize";
const O_AUTH_REDIRECT: &str = "http://localhost/auth/callback/spotify";

const MY_ID: &str = "5721ace651424098be643dfcf0533684";

const DESIRED_SCOPES: &str = "user-library-read";

#[tokio::main]
async fn main() -> Result<()> {
    let requester = Client::builder()
        .build()
        .context("Failed to initialize client side socket")?;

    if let Err(url) = requester
        .get(O_AUTH_ENDPOINT)
        .query(&hash_map! {
            "response_type" => "code",
            "client_id" => MY_ID,
            "redirect_uri" => O_AUTH_REDIRECT,
            "scope" => DESIRED_SCOPES,
        })
        .send()
        .await
        .context("Can't send the auth request")?
        .url()
        .open_in_browser()
    {
        eprintln!("Couldn't open browser, please head to {}", url);
    }

    Ok(())
}

trait Openable {
    fn open_in_browser(&self) -> Result<Output, &Self>;
}

impl Openable for Url {
    fn open_in_browser(&self) -> Result<Output, &Self> {
        webbrowser::open(self.as_str()).map_err(|_| self)
    }
}
