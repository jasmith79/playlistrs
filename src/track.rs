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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_artist_artist() {
        let track = Track {
            name: Some(String::from("Favorite Song")),
            location: Some(PathBuf::from("/foo/bar")),
            duration: 3000,
            artist: Some(String::from("bob")),
            album_artist: Some(String::from("steve")),
            composer: Some(String::from("jane")),
        };

        assert_eq!(track.get_artist(), "bob");
    }

    #[test]
    fn test_get_artist_album_artist() {
        let track = Track {
            name: Some(String::from("Favorite Song")),
            location: Some(PathBuf::from("/foo/bar")),
            duration: 3000,
            artist: None,
            album_artist: Some(String::from("bob")),
            composer: Some(String::from("steve")),
        };

        assert_eq!(track.get_artist(), "bob");
    }

    #[test]
    fn test_get_artist_composer() {
        let track = Track {
            name: Some(String::from("Favorite Song")),
            location: Some(PathBuf::from("/foo/bar")),
            duration: 3000,
            artist: None,
            album_artist: None,
            composer: Some(String::from("bob")),
        };

        assert_eq!(track.get_artist(), "bob");
    }

    #[test]
    fn test_get_artist_unknown() {
        let track = Track {
            name: Some(String::from("Favorite Song")),
            location: Some(PathBuf::from("/foo/bar")),
            duration: 3000,
            artist: None,
            album_artist: None,
            composer: None,
        };

        assert_eq!(track.get_artist(), "Unknown Artist");
    }

    #[test]
    fn test_get_name() {
        let track = Track {
            name: Some(String::from("Favorite Song")),
            location: Some(PathBuf::from("/foo/bar")),
            duration: 3000,
            artist: None,
            album_artist: None,
            composer: None,
        };

        assert_eq!(track.get_name(), "Favorite Song");
    }

    #[test]
    fn test_get_name_untitled() {
        let track = Track {
            name: None,
            location: Some(PathBuf::from("/foo/bar")),
            duration: 3000,
            artist: None,
            album_artist: None,
            composer: None,
        };

        assert_eq!(track.get_name(), "Untitled Track");
    }
}
