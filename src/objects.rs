// this is all extremely cyclical so I did not seperate it into modules

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TrackRestrictionObject {
    /// The reason for the restriction. Supported values:
    /// - `market` - The content item is not available in the given market.
    /// - `product` - The content item is not available for the user’s subscription type.
    /// - `explicit` - The content item is explicit and the user’s account is set to not play explicit content.
    /// Additional reasons may be added in the future.
    /// **Note**: If you use this field, make sure that your application safely handles unknown values.
    pub(crate) reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LinkedTrackObject {
    /// Known external URLs for this track.
    pub(crate) external_urls: Option<ExternalUrlObject>,
    /// A link to the Web API endpoint providing full details of the track.
    pub(crate) href: Option<String>,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the track.
    pub(crate) id: Option<String>,
    /// The object type: “track”.
    pub(crate) r#type: Option<String>,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the track.
    pub(crate) uri: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExternalIdObject {
    /// [International Article Number](http://en.wikipedia.org/wiki/International_Article_Number_%28EAN%29)
    pub(crate) ean: Option<String>,
    /// [International Standard Recording Code](http://en.wikipedia.org/wiki/International_Standard_Recording_Code)
    pub(crate) isrc: Option<String>,
    /// [Universal Product Code](http://en.wikipedia.org/wiki/Universal_Product_Code)
    pub(crate) upc: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FollowersObject {
    /// A link to the Web API endpoint providing full details of the followers; null if not available.
    /// Please note that this will always be set to null, as the Web API does not support it at the moment.
    pub(crate) href: Option<String>,
    /// The total number of followers.
    pub(crate) total: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct ArtistObject {
    /// Known external URLs for this artist.
    pub(crate) external_urls: Option<ExternalUrlObject>,
    /// Information about the followers of the artist.
    pub(crate) followers: Option<FollowersObject>,
    /// A list of the genres the artist is associated with.
    /// For example: `"Prog Rock"`, `"Post-Grunge"`. (If not yet classified, the array is empty).
    pub(crate) genres: Option<Vec<String>>,
    /// A link to the Web API endpoint providing full details of the artist.
    pub(crate) href: Option<String>,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the artist.
    pub(crate) id: Option<String>,
    /// Images of the artist in various sizes, widest first.
    pub(crate) images: Option<Vec<ImageObject>>,
    /// The name of the artist.
    pub(crate) name: String,
    /// The popularity of the artist.
    /// The value will be between 0 and 100, with 100 being the most popular.
    /// The artist’s popularity is calculated from the popularity of all the artist’s tracks.
    pub(crate) popularity: Option<u8>,
    /// The object type: "artist".
    pub(crate) r#type: Option<String>,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the artist.
    pub(crate) uri: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AlbumRestrictionObject {
    /// The reason for the restriction. Supported values:
    /// - `market` - The content item is not available in the given market.
    /// - `product` - The content item is not available for the user’s subscription type.
    /// - `explicit` - The content item is explicit and the user’s account is set to not play explicit content.
    /// Additional reasons may be added in the future.
    /// **Note**: If you use this field, make sure that your application safely handles unknown values.
    pub(crate) reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ImageObject {
    /// The image height in pixels. If unknown: `null` or not returned.
    pub(crate) height: Option<usize>,
    /// The source URL of the image.
    pub(crate) url: Option<String>,
    /// The image width in pixels. If unknown: `null` or not returned.
    pub(crate) width: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct ExternalUrlObject {
    /// The [Spotify URL](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the object.
    pub(crate) spotify: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SimplifiedArtistObject {
    /// Known external URLs for this artist.
    pub(crate) external_urls: Option<ExternalUrlObject>,
    /// A link to the Web API endpoint providing full details of the artist.
    pub(crate) href: Option<String>,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the artist.
    pub(crate) id: Option<String>,
    /// The name of the artist.
    pub(crate) name: Option<String>,
    /// The object type: "artist".
    pub(crate) r#type: Option<String>,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the artist.
    pub(crate) uri: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SimplifiedAlbumObject {
    /// The field is present when getting an artist’s albums.
    /// Possible values are “album”, “single”, “compilation”, “appears_on”.
    /// Compare to album_type this field represents relationship between the artist and the album.
    pub(crate) album_group: Option<String>,
    /// The type of the album: one of “album”, “single”, or “compilation”.
    pub(crate) album_type: Option<String>,
    /// The artists of the album.
    /// Each artist object includes a link in `href` to more detailed information about the artist.
    pub(crate) artists: Option<Vec<SimplifiedArtistObject>>,
    /// The markets in which the album is available: [ISO 3166-1 alpha-2 country codes](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2). Note that an album is considered available in a market when at least 1 of its tracks is available in that market.
    pub(crate) available_markets: Option<Vec<String>>,
    /// Known external URLs for this album.
    pub(crate) external_urls: Option<ExternalUrlObject>,
    /// A link to the Web API endpoint providing full details of the album.
    pub(crate) href: Option<String>,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the album.
    pub(crate) id: Option<String>,
    /// The cover art for the album in various sizes, widest first.
    pub(crate) images: Option<Vec<ImageObject>>,
    /// The name of the album. In case of an album takedown, the value may be an empty string.
    pub(crate) name: Option<String>,
    /// The date the album was first released, for example `1981`. Depending on the precision, it might be shown as `1981-12` or `1981-12-15`.
    pub(crate) release_date: Option<String>,
    /// The precision with which `release_date` value is known: `year`, `month`, or `day`.
    pub(crate) release_date_precision: Option<String>,
    /// Included in the response when a content restriction is applied. See [Restriction Object](https://developer.spotify.com/documentation/web-api/reference/#object-albumrestrictionobject) for more details.
    pub(crate) restrictions: Option<AlbumRestrictionObject>,
    /// The object type: “album”.
    pub(crate) r#type: Option<String>,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the album.
    pub(crate) uri: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TrackObject {
    /// The album on which the track appears.
    /// The album object includes a link in `href` to full information about the album.
    pub(crate) album: Option<SimplifiedAlbumObject>,
    /// The artists who performed the track.
    /// Each artist object includes a link in `href` to more detailed information about the artist.
    pub(crate) artists: Vec<ArtistObject>,
    /// A list of the countries in which the track can be played, identified by their [ISO 3166-1 alpha-2](http://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) code.
    pub(crate) available_markets: Option<Vec<String>>,
    /// The disc number (usually `1` unless the album consists of more than one disc).
    pub(crate) disc_number: Option<u8>,
    /// The track length in milliseconds.
    pub(crate) duration_ms: Option<usize>,
    /// Whether or not the track has explicit lyrics (`true` = yes it does; `false` = no it does not OR unknown).
    pub(crate) explicit: Option<bool>,
    /// Known external IDs for the track.
    pub(crate) external_ids: Option<ExternalIdObject>,
    /// Known external URLs for this track.
    pub(crate) external_urls: Option<ExternalUrlObject>,
    /// A link to the Web API endpoint providing full details of the track.
    pub(crate) href: Option<String>,
    /// The [Spotify ID](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the track.
    pub(crate) id: Option<String>,
    /// Whether or not the track is from a local file.
    pub(crate) is_local: Option<bool>,
    /// Part of the response when [Track Relinking](https://developer.spotify.com/documentation/general/guides/track-relinking-guide/) is applied.
    /// If `true`, the track is playable in the given market. Otherwise `false`.
    pub(crate) is_playable: Option<bool>,
    /// Part of the response when [Track Relinking](https://developer.spotify.com/documentation/general/guides/track-relinking-guide/) is applied, and the requested track has been replaced with different track.
    /// The track in the `linked_from` object contains information about the originally requested track.
    pub(crate) linked_from: Option<LinkedTrackObject>,
    /// The name of the track.
    pub(crate) name: String,
    /// The popularity of the track.
    /// The value will be between 0 and 100, with 100 being the most popular.
    /// The popularity is calculated by algorithm and is based, in the most part,
    /// on the total number of plays the track has had and how recent those plays are.
    /// Generally speaking, songs that are being played a lot now will have a higher popularity than songs that were played a lot in the past.
    /// Duplicate tracks (e.g. the same track from a single and an album) are rated independently.
    /// Artist and album popularity is derived mathematically from track popularity.
    /// Note that the popularity value may lag actual popularity by a few days: the value is not updated in real time.
    pub(crate) popularity: Option<u8>,
    /// A link to a 30 second preview (MP3 format) of the track. Can be `null`.
    pub(crate) preview_url: Option<String>,
    /// Included in the response when a content restriction is applied. See [Restriction Object](https://developer.spotify.com/documentation/web-api/reference/#object-trackrestrictionobject) for more details.
    pub(crate) restrictions: Option<TrackRestrictionObject>,
    /// The number of the track. If an album has several discs, the track number is the number on the specified disc.
    pub(crate) track_number: Option<u8>,
    /// The object type: “track”.
    pub(crate) r#type: Option<String>,
    /// The [Spotify URI](https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids) for the track.
    pub(crate) uri: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SavedTrackObject {
    /// The date and time the track was saved.
    /// Timestamps are returned in ISO 8601 format as Coordinated Universal Time (UTC) with a zero offset: YYYY-MM-DDTHH:MM:SSZ.
    /// If the time is imprecise (for example, the date/time of an album release), an additional field indicates the precision;
    /// see for example, release_date in an album object.
    pub(crate) added_at: Option<DateTime<Utc>>,
    /// Information about the track.
    pub(crate) track: TrackObject,
}

#[derive(Debug, Deserialize)]
pub struct PagingObject<T> {
    /// A link to the Web API endpoint returning the full result of the request.
    pub(crate) href: Option<String>,
    /// The requested data.
    pub(crate) items: Vec<T>,
    /// The maximum number of items in the response (as set in the query or by default).
    pub(crate) limit: u8,
    /// URL to the next page of items. (`null` if none).
    pub(crate) next: Option<String>,
    /// The offset of the items returned (as set in the query or by default).
    pub(crate) offset: usize,
    /// URL to the previous page of items. (`null` if none)
    pub(crate) previous: Option<String>,
    /// The total number of items available to return.
    pub(crate) total: usize,
}
