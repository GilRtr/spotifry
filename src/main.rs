#![feature(try_blocks, type_ascription, once_cell)]

use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
    lazy::Lazy,
    net::Ipv4Addr,
    process::Output,
};

use anyhow::{Context, Result};
use common_macros::hash_map;
use objects::{PagingObject, SavedTrackObject, SimplifiedPlaylistObject};
use reqwest::{Client, Url};
use serde::Deserialize;
use tio::AsyncBufReadExt;
use tokio::{
    io::{self as tio, AsyncReadExt},
    net::{TcpListener, TcpStream},
};

mod objects;

const O_AUTH_ENDPOINT: &str = "https://accounts.spotify.com/authorize";
const O_AUTH_REDIRECT: &str = "http://localhost/auth/callback/spotify";

const MY_ID: Lazy<String> = Lazy::new(|| {
    println!("enter your ID");
    let mut buf = String::new();
    BufReader::new(io::stdin()).read_line(&mut buf);
    buf.trim().to_owned()
});
const MY_SECRET: Lazy<String> = Lazy::new(|| {
    println!("enter your Secret");
    let mut buf = String::new();
    BufReader::new(io::stdin()).read_line(&mut buf);
    buf.trim().to_owned()
});

const DESIRED_SCOPES: &str = "user-library-read playlist-modify-private playlist-read-private";

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

    let tracks = get_tracks(&requester, &tokens.access_token).await?;

    let mut playlists = get_playlists_request(&requester, &tokens.access_token, 50, 0).await?;
    let mut offset = 0;
    while offset < playlists.total {
        playlists
            .items
            .iter()
            .enumerate()
            .for_each(|(i, x)| println!("[{}] - {}", i + offset, x.name));

        offset += playlists.limit as usize;

        playlists = get_playlists_request(&requester, &tokens.access_token, 50, offset).await?;
    }
    playlists
        .items
        .iter()
        .enumerate()
        .for_each(|(i, x)| println!("[{}] - {}", i + offset, x.name));
    let n: usize = {
        let mut dst = String::new();
        tio::BufReader::new(tio::stdin())
            .read_line(&mut dst)
            .await?;
        let n = dst.parse().context("Enter a number!")?;
        n
    };

    add_to_playlist(
        &requester,
        &tokens.access_token,
        &playlists.items[n].id,
        // "621c621e1ef54d91",
        &tracks,
    )
    .await?;

    Ok(())
}

async fn add_to_playlist(
    requester: &Client,
    token: &str,
    playlist_id: &str,
    tracks: &Vec<SavedTrackObject>,
) -> Result<()> {
    let limit = 100;
    let mut offset = 0;

    while offset < tracks.len() {
        add_to_playlist_request(
            requester,
            token,
            playlist_id,
            &tracks
                .iter()
                .skip(offset)
                .take(limit)
                .map(|track| &track.track.uri)
                .collect(),
        )
        .await?;
        offset += limit;
    }

    Ok(())
}

async fn get_playlists_request(
    requester: &Client,
    token: &str,
    limit: u8,
    offset: usize,
) -> Result<PagingObject<SimplifiedPlaylistObject>> {
    requester
        .get("https://api.spotify.com/v1/users/me/playlists")
        .header("Authorization", format!("Bearer {}", token))
        .query(&hash_map! {
            "limit" => limit as usize,
            "offset" => offset,
        })
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
        .context("Can't parse playlists to json")
}

async fn add_to_playlist_request(
    requester: &Client,
    token: &str,
    playlist_id: &str,
    track_uris: &Vec<&String>,
) -> Result<()> {
    requester
        .post(&format!(
            "https://api.spotify.com/v1/playlists/{playlist_id}/tracks",
            playlist_id = playlist_id
        ))
        .header("Authorization", format!("Bearer {}", token))
        .json(track_uris)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

async fn get_tracks(requester: &Client, token: &str) -> Result<Vec<SavedTrackObject>> {
    let mut offset = 0;
    let mut limit = 50;
    let mut tracks = get_tracks_request(requester, token, offset, limit)
        .await
        .context("Request for tracks has failed")?;
    let mut all_tracks = Vec::with_capacity(tracks.total);
    all_tracks.extend(tracks.items);

    while tracks.offset < tracks.total {
        offset += limit as usize;

        tracks = get_tracks_request(requester, token, offset, limit)
            .await
            .context("Request for tracks has failed")?;

        limit = tracks.limit;

        all_tracks.extend(tracks.items);
    }

    Ok(all_tracks)
}

async fn get_tracks_request(
    requester: &Client,
    token: &str,
    offset: usize,
    limit: u8,
) -> Result<PagingObject<SavedTrackObject>> {
    requester
        .get("https://api.spotify.com/v1/me/tracks")
        .header("Authorization", format!("Bearer {}", token))
        .query(&hash_map! {
            "offset" => offset,
            "limit" => limit as _,
        })
        .send()
        .await?
        .error_for_status()?
        .json::<PagingObject<SavedTrackObject>>()
        .await
        .context("Can't parse tracks to json")
}

#[allow(dead_code)]
async fn refresh_tokens(requester: &Client, refresh_token: &str) -> Result<Tokens> {
    acquire_tokens(
        requester,
        &Data {
            client_id: &MY_ID,
            client_secret: &MY_SECRET,
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
            client_id: &MY_ID,
            client_secret: &MY_SECRET,
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
    let id = &MY_ID;
    if let Err((url, _io_error)) = requester
        .get(O_AUTH_ENDPOINT)
        .query(&hash_map! {
            "response_type" => "code",
            "client_id" => id,
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
