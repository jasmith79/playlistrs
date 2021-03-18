//! main
//!
//! Main file for the Apple playlist extractor.
//!
//! @author jasmith79
//! @license MIT
//! @copyright 2021
mod cli;
mod m3u;
mod read_library_xml;
mod track_record;

use regex::Regex;
use std::fs;

fn main() {
    // If Apple changes this again move to switch statement rather than
    // making the regex scarier.
    let old_path = Regex::new(r"[\\/]Users[\\/][^\\/]+[\\/]Music[\\/](?:Music|iTunes)[\\/](?:iTunes(?: |%20)Media|Media\.localized)")
        .expect("Could not create Regex for music path matching");

    let parsed_args = cli::parse_args();
    let xml_data = read_library_xml::read_library(parsed_args.path);
    let playlists = read_library_xml::extract_playlists(xml_data);
    for (name, mut tracks) in playlists {
        if !parsed_args.music_path.is_empty() {
            for track in &mut tracks {
                let stripped = old_path.replace_all(&track.location, "");
                track.location = format!("{}{}", parsed_args.music_path, stripped);
            }
        }
        let playlist = m3u::convert(&name, &tracks);
        let file_name = format!("{}.m3u", name);
        let write_path = parsed_args.output_path.join(file_name);
        fs::write(write_path, playlist).expect("Couldn't write file");
    }
}
