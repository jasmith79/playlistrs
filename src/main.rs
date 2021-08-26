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

use std::fs;

use crate::track_record::{path_to_file_url, calc_new_location};

fn main() {
    let parsed_args = cli::parse_args();
    let xml_data = read_library_xml::read_library(parsed_args.path);
    let playlists = read_library_xml::extract_playlists(&xml_data);
    for (name, mut tracks) in playlists {
        if !parsed_args.music_path.is_empty() {
            for track in &mut tracks {
                let updated = calc_new_location(&track.location, &parsed_args.music_path);
                track.location = if parsed_args.use_file_url {
                    path_to_file_url(&updated)
                } else {
                    updated.to_owned()
                };
            }
        }
        let playlist = m3u::convert(&name, &tracks);
        let file_name = format!("{}.m3u", name);
        let write_path = parsed_args.output_path.join(file_name);
        fs::write(write_path, playlist).expect("Couldn't write file");
    }
}
