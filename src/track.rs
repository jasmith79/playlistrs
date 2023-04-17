//! track
//!
//! Defines the TrackRecord struct for data extracted from the Library
//! XML file.
//! author: jasmith79
//! license: MIT
//! copyright: 2023
use crate::path_utils::{deserialize_and_normalize, deserialize_path};
use serde::Deserialize;
use std::path::PathBuf;

static UNTITLED_TRACK: &str = "Untitled Track";
static UNKNOWN_ARTIST: &str = "Unknown Artist";

#[derive(Deserialize)]
pub struct Track {
    // May need this for xspf at some point
    // #[serde(rename = "Track ID")]
    // track_id: i32,
    #[serde(rename = "Name")]
    #[serde(deserialize_with = "deserialize_and_normalize")]
    #[serde(default)]
    pub name: Option<String>,

    #[serde(rename = "Location")]
    #[serde(deserialize_with = "deserialize_path")]
    #[serde(default)]
    pub location: Option<PathBuf>,

    #[serde(rename = "Total Time")]
    #[serde(default)]
    pub duration: i32,

    // May need this for xspf at some point
    // #[serde(rename = "Album")]
    // #[serde(default)]
    // album: Option<String>,
    #[serde(rename = "Album Artist")]
    #[serde(deserialize_with = "deserialize_and_normalize")]
    #[serde(default)]
    pub album_artist: Option<String>,

    #[serde(rename = "Artist")]
    #[serde(deserialize_with = "deserialize_and_normalize")]
    #[serde(default)]
    pub artist: Option<String>,

    #[serde(rename = "Composer")]
    #[serde(deserialize_with = "deserialize_and_normalize")]
    #[serde(default)]
    pub composer: Option<String>,
}

impl Track {
    pub fn get_artist(&self) -> &str {
        self.artist
            .as_deref()
            .or(self.album_artist.as_deref())
            .or(self.composer.as_deref())
            .unwrap_or(UNKNOWN_ARTIST)
    }

    pub fn get_name(&self) -> &str {
        self.name.as_deref().unwrap_or(UNTITLED_TRACK)
    }
}
