//! library_xml
//!
//! Contains functions pertaining to the manipulation of Apple's music
//! library XML files.
//!
//! @author jasmith79
//! @license MIT
//! @copyright 2023
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;

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

pub fn get_itunes_prefix(lib_xml: &LibraryXMLData) -> Result<&Path> {
    let first_list = &lib_xml.playlists[0];
    let first_track_record = &first_list.playlist_items[0];
    let first_id = first_track_record.track_id.to_string();
    let first_track = lib_xml
        .tracks
        .get(&first_id)
        .context("Cannot find first track")?;
    let first_location = first_track
        .location
        .as_ref()
        .context("First track has no location")?;
    let prefix = generate_itunes_prefix(first_location)?;
    Ok(prefix)
}
