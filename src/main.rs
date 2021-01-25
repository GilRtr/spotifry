#![cfg_attr(feature = "nightly", feature(try_blocks))]
#![cfg_attr(feature = "nightly", feature(type_ascription))]

use std::{io, net::Ipv4Addr, process::Output};

use anyhow::{Context, Result};
use common_macros::hash_map;
use reqwest::{Client, Url};
use tokio::{
    io::{self as tio, AsyncReadExt},
    net::{TcpListener, TcpStream},
};

const O_AUTH_ENDPOINT: &str = "https://accounts.spotify.com/authorize";
const O_AUTH_REDIRECT: &str = "http://localhost/auth/callback/spotify";

const MY_ID: &str = "5721ace651424098be643dfcf0533684";

const DESIRED_SCOPES: &str = "user-library-read";

#[tokio::main]
async fn main() -> Result<()> {
    let requester = Client::builder()
        .build()
        .context("Failed to initialize client side socket")?;

    let code = authorize_scope(requester)
        .await
        .context("Scope authorization has failed")?;

    println!("{:?}", String::from_utf8_lossy(&code));

    Ok(())
}

#[cfg(not(feature = "nightly"))]
async fn authorize_scope(requester: Client) -> Result<[u8; 210]> {
    let redirect_listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 80)).await;

    ask_for_authorization(requester)
        .await
        .context("Failed to ask for your authentication")?;

    let mut buf = [0; 210];
    {
        async fn __try(buf: &mut [u8], redirect_listener: io::Result<TcpListener>) -> Result<()> {
            match {
                async fn __try(redirect_listener: io::Result<TcpListener>) -> Result<TcpStream> {
                    let (mut stream, _sender) = redirect_listener?.accept().await?;
                    stream.read_exact(&mut [0; 32]).await?;
                    Ok(stream)
                }
                __try(redirect_listener).await
            } {
                Ok(mut stream) => stream.read_exact(buf).await,
                Err(_) => {
                    let mut stream = tio::stdin();
                    println!("Please enter the URL you were redirected to:");
                    stream.read_exact(&mut [0; 44]).await?;
                    stream.read_exact(buf).await
                }
            }?;
            Ok(())
        }
        __try(&mut buf, redirect_listener).await
    }
    .context("Failed to read the authentication code")?;

    Ok(buf)
}

#[cfg(feature = "nightly")]
async fn authorize_scope(requester: Client) -> Result<[u8; 210]> {
    let redirect_listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 80)).await;

    ask_for_authorization(requester)
        .await
        .context("Failed to ask for your authentication")?;

    let mut buf = [0; 210];

    (try {
        match try {
            let (mut stream, _sender) = redirect_listener?.accept().await?;
            stream.read_exact(&mut [0; 32]).await?;
            stream
        }: Result<TcpStream>
        {
            Ok(mut stream) => stream.read_exact(&mut buf).await,
            Err(_) => {
                let mut stream = tio::stdin();
                stream.read_exact(&mut [0; 5]).await?;
                stream.read_exact(&mut buf).await
            }
        }?;
    }: Result<()>)
        .context("Failed to read the authentication code")?;

    Ok(buf)
}

async fn ask_for_authorization(requester: Client) -> Result<()> {
    if let Err((url, _io_error)) = requester
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
    fn open_in_browser(&self) -> Result<Output, (&Self, io::Error)>;
}

impl Openable for Url {
    fn open_in_browser(&self) -> Result<Output, (&Self, io::Error)> {
        webbrowser::open(self.as_str()).map_err(|e| (self, e))
    }
}
