#![cfg_attr(feature = "nightly", feature(try_blocks, type_ascription))]

use std::{collections::HashMap, io, net::Ipv4Addr, process::Output};

use anyhow::{Context, Result};
use common_macros::hash_map;
use reqwest::{Client, Url};
use serde::Deserialize;
use tokio::{
    io::{self as tio, AsyncReadExt},
    net::{TcpListener, TcpStream},
};

const O_AUTH_ENDPOINT: &str = "https://accounts.spotify.com/authorize";
const O_AUTH_REDIRECT: &str = "http://localhost/auth/callback/spotify";

const MY_ID: &str = "5721ace651424098be643dfcf0533684";
const MY_SECRET: &str = "8cac62a5509d4008829f3c938455914d";

const DESIRED_SCOPES: &str = "user-library-read";

const ACCOUNTS_SERVICE: &str = "https://accounts.spotify.com/api/token";

#[tokio::main]
async fn main() -> Result<()> {
    let requester = Client::builder()
        .build()
        .context("Failed to initialize client side socket")?;

    let authorization_code = authorize_scope(&requester)
        .await
        .context("Scope authorization has failed")?;

    let authorization_code = String::from_utf8_lossy(&authorization_code);

    let tokens = get_tokens(&requester, &authorization_code)
        .await
        .context("Failed to retrieve refresh and access tokens")?;

    println!("{:?}", tokens);

    Ok(())
}

#[allow(dead_code)]
async fn refresh_tokens(requester: &Client, refresh_token: &str) -> Result<Tokens> {
    acquire_tokens(
        requester,
        &Data {
            client_id: MY_ID,
            client_secret: MY_SECRET,
            prior_data: Refresh { refresh_token },
        },
    )
    .await
    .context("Failed to refresh tokens")
}

async fn get_tokens(requester: &Client, authorization_code: &str) -> Result<Tokens> {
    acquire_tokens(
        requester,
        &Data {
            client_id: MY_ID,
            client_secret: MY_SECRET,
            prior_data: Initial {
                redirect_uri: O_AUTH_REDIRECT,
                code: authorization_code,
            },
        },
    )
    .await
    .context("Failed to aquire initial tokens")
}

async fn acquire_tokens(requester: &Client, prior_data: &Data<'_, '_, '_>) -> Result<Tokens> {
    requester
        .post(ACCOUNTS_SERVICE)
        .form(&HashMap::from(prior_data))
        .send()
        .await
        .context("The request for tokens has gone kaput")?
        .error_for_status()?
        .json::<Tokens>()
        .await
        .context("Couldn't deserialize the response into valid JSON")
}

struct Data<'prior, 'id, 'secret> {
    client_id: &'id str,
    client_secret: &'secret str,
    prior_data: PriorData<'prior>,
}

enum PriorData<'prior> {
    Initial {
        code: &'prior str,
        redirect_uri: &'prior str,
    },
    Refresh {
        refresh_token: &'prior str,
    },
}

use PriorData::{Initial, Refresh};

impl<'prior, 'id, 'secret> From<&Data<'prior, 'id, 'secret>> for HashMap<&'static str, &'prior str>
where
    'id: 'prior,
    'secret: 'prior,
{
    fn from(prior_data: &Data<'prior, 'id, 'secret>) -> Self {
        match prior_data {
            Data {
                client_id,
                client_secret,
                prior_data: PriorData::Initial { code, redirect_uri },
            } => hash_map! {
                "client_id" => *client_id,
                "client_secret" => *client_secret,
                "grant_type" => "authorization_code",
                "code" => *code,
                "redirect_uri" => *redirect_uri,
            },
            Data {
                client_id,
                client_secret,
                prior_data: PriorData::Refresh { refresh_token },
            } => hash_map! {
                "client_id" => *client_id,
                "client_secret" => *client_secret,
                "grant_type" => "refresh_token",
                "refresh_token" => *refresh_token,
            },
        }
        // }
    }
}

#[derive(Debug, Deserialize)]
struct Tokens {
    /// An access token that can be provided in subsequent calls, for example to Spotify Web API services.
    access_token: String,
    /// How the access token may be used: always “Bearer”.
    token_type: String,
    /// A space-separated list of scopes which have been granted for this `access_token`.
    scope: String,
    /// The time period (in seconds) for which the access token is valid.
    expires_in: usize,
    /// A token that can be sent to the Spotify Accounts service in place of an authorization code.
    /// When the access code expires, send a POST request to the Accounts service `/api/token` endpoint, but use this code in place of an authorization code.
    /// A new access token will be returned. A new refresh token might be returned too.
    refresh_token: String,
}

/// The user is asked to authorize access.
/// The user is redirected to `REDIRECT_URI`.
#[cfg(not(feature = "nightly"))]
async fn authorize_scope(requester: &Client) -> Result<[u8; 210]> {
    let redirect_listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 80)).await;

    ask_for_authorization(requester)
        .await
        .context("Failed to ask for your authorization")?;

    let mut buf = [0; 210];
    {
        async fn __try(buf: &mut [u8], redirect_listener: io::Result<TcpListener>) -> Result<()> {
            match {
                async fn __try(redirect_listener: io::Result<TcpListener>) -> Result<TcpStream> {
                    // TODO: solve with actix-web and one-shot and handle error
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
    .context("Failed to read the authorization code")?;

    Ok(buf)
}

/// The user is asked to authorize access.
/// The user is redirected to `REDIRECT_URI`.
#[cfg(feature = "nightly")]
async fn authorize_scope(requester: &Client) -> Result<[u8; 210]> {
    let redirect_listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 80)).await;

    ask_for_authorization(requester)
        .await
        .context("Failed to ask for your authorization")?;

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
        .context("Failed to read the authorization code")?;

    Ok(buf)
}

/// Request authorization; the user logs in and authorizes access.
/// I send a request to the Spotify Accounts service.
/// The user is asked to authorize access within the scopes.
/// - If the user is not logged in, they are prompted to do so using their Spotify credentials.
/// - When the user is logged in, they are asked to authorize access to the data sets defined in the scopes.
async fn ask_for_authorization(requester: &Client) -> Result<()> {
    if let Err((url, _io_error)) = requester
        .get(O_AUTH_ENDPOINT)
        .query(&hash_map! {
            "response_type" => "code",
            "client_id" => MY_ID,
            "redirect_uri" => O_AUTH_REDIRECT,
            "scope" => DESIRED_SCOPES,
        }) // TODO: Add state
        .send()
        .await
        .context("Can't send the auth request")?
        .error_for_status()?
        .url()
        .open_in_browser()
    {
        // TODO: replace with log
        eprintln!("Couldn't open browser, please head to {}", url);
    }
    Ok(())
}

/// Somthing that can be opened in a browser.
trait Openable {
    /// The string representation of the URL
    fn url(&self) -> &str;
    /// Opens the url in default web browser
    fn open_in_browser(&self) -> Result<Output, (&Self, io::Error)> {
        webbrowser::open(self.url()).map_err(|e| (self, e))
    }
}

impl Openable for Url {
    fn url(&self) -> &str {
        self.as_str()
    }
}
