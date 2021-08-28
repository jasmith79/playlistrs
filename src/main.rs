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

fn main() {
    let parsed_args = cli::get_args();
    let xml_data = read_library_xml::read_library(&parsed_args.path);
    let playlists = read_library_xml::extract_playlists(&xml_data);
    for (name, tracks) in playlists {
        let playlist = m3u::convert(&name, &tracks);
        let file_name = format!("{}.m3u", name);
        let write_path = parsed_args.output_path.join(file_name);
        fs::write(write_path, playlist).expect("Couldn't write file");
    }
}
