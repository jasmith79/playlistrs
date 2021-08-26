//! read_library_xml
//!
//! Contains functions pertaining to the manipulation of Apple's music
//! library XML files.
//!
//! @author jasmith79
//! @license MIT
//! @copyright 2021

use plist::{Dictionary, Value};
use std::path::PathBuf;
use unicode_normalization::UnicodeNormalization;

use crate::track_record::{TrackRecord, file_url_to_path};

/// Reads the XML Library file into a data structure in memory.
pub fn read_library(file: PathBuf) -> Dictionary {
    let parsed = Value::from_file(file).expect("failed to read Library XML");
    let dict = parsed
        .as_dictionary()
        .expect("XML data not in expected key/value format.")
        .to_owned();

    return dict;
}

pub fn extract_track_data(track: &Dictionary) -> TrackRecord {
    let track_id = track
        .get("Track ID")
        .expect("Track has no Track ID!")
        .as_string()
        .unwrap_or("");

    let name = track
        .get("Name")
        .unwrap_or(&Value::String(String::from("Untitled")))
        .as_string()
        .expect(&format!(
            "Track {} Name value present but is not a string",
            track_id
        ))
        .nfc()
        .to_string();

    let location = file_url_to_path(track
        .get("Location")
        .expect(&format!("Track {}/{} has no Location!", track_id, name))
        .as_string()
        .expect(&format!(
            "Track {}/{} location is not a string.",
            track_id, name
        ))
    );

    let duration_ms = track
        .get("Total Time")
        .expect(&format!("Track {}/{} has no run time.", track_id, name))
        .as_signed_integer()
        .expect(&format!(
            "Track {}/{} run time is not an integer.",
            track_id, name
        ));

    let album = track
        .get("Album")
        .unwrap_or(&Value::String(String::from("")))
        .as_string()
        .expect(&format!(
            "Track {}/{} Album value present but is not a string",
            track_id, name
        ))
        .nfc()
        .to_string();

    let artist = track
        .get("Artist")
        .unwrap_or(&Value::String(String::from("")))
        .as_string()
        .expect(&format!(
            "Track {}/{} Artist value present but is not a string",
            track_id, name
        ))
        .nfc()
        .to_string();

    let album_artist = track
        .get("Album Artist")
        .unwrap_or(&Value::String(String::from("")))
        .as_string()
        .expect(&format!(
            "Track {}/{} Album Artist value present but is not a string",
            track_id, name
        ))
        .nfc()
        .to_string();

    let composer = track
        .get("Composer")
        .unwrap_or(&Value::String(String::from("")))
        .as_string()
        .expect(&format!(
            "Track {}/{} Composer value present but is not a string",
            track_id, name
        ))
        .nfc()
        .to_string();

    return TrackRecord {
        location,
        duration_ms,
        album,
        name,
        artist,
        album_artist,
        composer,
    };
}

pub fn valid_track(track: &&Dictionary) -> bool {
    let track_id = track
        .get("Track ID")
        .expect("Track has no Track ID!")
        .as_string()
        .unwrap_or("");

    let empty_value = Value::String(String::from(""));
    let track_name = track
        .get("Name")
        .unwrap_or(&empty_value)
        .as_string()
        .expect(&format!("Track {} Name is not a string", track_id));

    let passes = track.contains_key("Location");
    let identifier = if track_name.is_empty() {
        track_id
    } else {
        track_name
    };
    if !passes {
        println!("WARNING: Track {} has no Location! Skipping...", identifier);
    }

    return passes;
}

pub fn extract_playlist_data<'a>(
    all_tracks: &'a Dictionary,
    play: &Dictionary,
) -> (String, Vec<TrackRecord>) {
    let trks = play
        .get("Playlist Items")
        .expect("No items in playlist!")
        .as_array()
        .expect("Playlist items not in expected array format.")
        .iter()
        .map(|trck| {
            let track_id = trck
                .as_dictionary()
                .expect("Playlist track does not have a Track ID.")
                .get("Track ID")
                .expect("Playlist track Track ID not in expected key/value format")
                .as_signed_integer()
                .expect("Playlist track Track ID is not an integer value.")
                .to_string();

            let track = all_tracks
                .get(&track_id)
                .expect(&format!("No track present with ID {}", track_id))
                .as_dictionary()
                .expect(&format!(
                    "Track {} did not have expected key/value format",
                    track_id
                ));

            return track;
        })
        .filter(valid_track)
        .map(extract_track_data)
        .collect();

    return (
        play.get("Name")
            .expect("Playlist has no name!")
            .as_string()
            .expect("Playlist has non-string Name!")
            .to_owned(),
        trks,
    );
}

/// Extracts playlist information from the XML data.
pub fn extract_playlists<'a>(xml_data: &'a Dictionary) -> Vec<(String, Vec<TrackRecord>)> {
    let tracks = &xml_data
        .get("Tracks")
        .expect("No tracks present in Library XML")
        .as_dictionary()
        .expect("Tracks in Library XML not in expected key/value format.");

    let playlists: Vec<(String, Vec<TrackRecord>)> = xml_data
        .get("Playlists")
        .expect("No playlists present!")
        .as_array()
        .expect("Playlists not in expected array format.")
        .iter()
        // Yes, I know could condense the next few calls to a single fold.
        .map(|x| {
            x.as_dictionary()
                .expect("Playlist not in expected key/value format")
        })
        // This filter removes the autogenerated playlists like 'Recently Added',
        // which for whatever reason have missing tracks or tracks with too many
        // missing properties.
        // TODO: debug add cli switch to toggle?
        .filter(|play| !(play.contains_key("Master") || play.contains_key("Distinguished Kind")))
        // Tried and failed to make this return a closure to get a lil closer to point-free.
        // TODO: figure out how to partially apply the tracks param
        .map(|play| extract_playlist_data(tracks, play))
        .collect();

    return playlists;
}
