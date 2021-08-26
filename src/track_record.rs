//! track_record
//!
//! Defines the TrackRecord struct for data extracted from the Library
//! XML file.
//! @author jasmith79
//! @license MIT
//! @copyright 2021
use std::path::Path;
use lazy_static::lazy_static;
use urlencoding::{decode, encode};
use unicode_normalization::UnicodeNormalization;
use regex::Regex;
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

pub fn file_url_to_path(file_url: &str) -> String {
    let err_msg = &format!(
        "Location {} is not UTF-8 encoded.",
        file_url,
    );

    return decode(&file_url)
        .expect(err_msg)
        .nfc()
        // TODO: got to be a better way to get a substr...
        .to_string()[7..]
        .to_string();
}

pub fn path_to_file_url(path: &str) -> String {
    let encoded = encode(&path);
    return format!("file://{}", encoded);
}

pub fn calc_new_location(old_path: &str, new_path: &str) -> String {
    lazy_static! {
        // If Apple changes this again move to switch statement rather than
        // making the regex scarier.
        static ref APPLE_PATH: Regex = Regex::new(r"[\\/]Users[\\/][^\\/]+[\\/]Music[\\/](?:Music|iTunes)[\\/](?:iTunes(?: |%20)Media|Media\.localized)")
        .expect("Could not create Regex for music path matching");
    }
    
    let stripped = APPLE_PATH.replace_all(old_path, "").to_string();
    // TODO: add better error logic for non unicode characters, i.e. replace
    // to_string_lossy
    return Path::new(new_path).join(stripped).to_string_lossy().to_string();
}
