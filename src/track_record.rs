//! track_record
//!
//! Defines the TrackRecord struct for data extracted from the Library
//! XML file.
//! author: jasmith79
//! license: MIT
//! copyright: 2021
use lazy_static::lazy_static;
use regex::Regex;
use std::path::{self, MAIN_SEPARATOR, Path};
use unicode_normalization::UnicodeNormalization;
use urlencoding::{decode, encode};

use crate::cli;
#[derive(Debug, Clone)]
pub struct TrackRecord {
    pub location: String,
    pub duration_ms: i64,
    pub album: String,
    pub name: String,
    pub artist: String,
    pub album_artist: String,
    pub composer: String,
}

impl TrackRecord {
    pub fn new(
        location: &str,
        duration_ms: i64,
        album: String,
        name: String,
        artist: String,
        album_artist: String,
        composer: String,
    ) -> TrackRecord {
        return TrackRecord {
            location: file_url_to_path(location),
            duration_ms,
            album,
            name,
            artist,
            album_artist,
            composer,
        }
    }

    pub fn update_location(&mut self, new_path: &str) -> &TrackRecord {
        lazy_static! {
            // If Apple changes this again move to switch statement rather than
            // making the regex scarier.
            static ref APPLE_PATH: Regex = Regex::new(r"[\\/]Users[\\/][^\\/]+[\\/]Music[\\/](?:Music|iTunes)[\\/](?:iTunes(?: |%20)Media|Media\.localized)\\/")
            .expect("Could not create Regex for music path matching");
        }

        let stripped = APPLE_PATH.replace_all(&self.location, "").to_string();
        let location = Path::new(new_path)
            .join(stripped)
            .to_string_lossy()
            .to_string();
        
        self.location = if cli::get_args().use_file_url {
            path_to_file_url(&location)
        } else {
            location
        };
        
        return self;
    }
}

/// Converts a file URL to a bare path string (&str). Inverse of
/// path_to_file_url.
///
/// # Example
///
/// ```
/// let p = file_url_to_path("file:///path/to/some%20file.txt");
/// assert_eq!(p, "/path/to/some file.txt");
/// ```
pub fn file_url_to_path(file_url: &str) -> String {
    let err_msg = &format!("Location {} is not UTF-8 encoded.", file_url,);

    return decode(&file_url)
        .expect(err_msg)
        .nfc()
        // TODO: got to be a better way to get a substr...
        .to_string()[7..]
        .to_string();
}

/// Converts a path (as a &str) to a file URL. Inverse of
/// file_url_to_path.
///
/// # Example
///
/// ```
/// let p = path_to_file_url("/path/to/some file.txt");
/// assert_eq!(p, "file:///path/to/some%20file.txt");
/// ```
pub fn path_to_file_url(path: &str) -> String {
    let encoded = path
        .split(MAIN_SEPARATOR)
        .collect::<Vec<&str>>()
        .iter()
        .map(|piece| {
            return encode(piece).to_string();
        })
        .collect::<Vec<String>>()
        // I know this is the 'wrong' way to do this
        // vs the PathBuf .join, but #whateves
        .join(&MAIN_SEPARATOR.to_string());

    return format!("file://{}", encoded);
}
