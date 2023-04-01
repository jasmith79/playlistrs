//! library_xml
//!
//! Contains functions pertaining to the manipulation of Apple's music
//! library XML files.
//!
//! @author jasmith79
//! @license MIT
//! @copyright 2023
use std::collections::HashMap;
use std::path::Path;

use plist::from_file;
use serde::Deserialize;

use crate::path_utils::generate_itunes_prefix;
use crate::track::Track;

#[derive(Deserialize)]
pub struct PlaylistItem {
    #[serde(rename = "Track ID")]
    pub track_id: i32,
}

#[derive(Deserialize)]
pub struct Playlist {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Playlist Items")]
    pub playlist_items: Vec<PlaylistItem>,
}

#[derive(Deserialize)]
pub struct LibraryXMLData {
    #[serde(rename = "Tracks")]
    pub tracks: HashMap<String, Track>,

    #[serde(rename = "Playlists")]
    pub playlists: Vec<Playlist>,
}

pub fn read_xml<P>(path: &P) -> LibraryXMLData
where
    P: AsRef<Path>,
{
    let res = from_file::<&P, LibraryXMLData>(path);
    if res.is_err() {
        panic!("Could not load playlist data from the given file.")
    }

    res.unwrap()
}

pub fn get_itunes_prefix(lib_xml: &LibraryXMLData) -> &Path {
    let first_list = &lib_xml.playlists[0];
    let first_track_record = &first_list.playlist_items[0];
    let first_id = first_track_record.track_id.to_string();
    let first_track = lib_xml.tracks.get(&first_id);
    let first_location = first_track.unwrap().location.as_ref().unwrap();
    generate_itunes_prefix(first_location).unwrap()
}
