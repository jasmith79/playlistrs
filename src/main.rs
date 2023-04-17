//! main
//!
//! Main file for the Apple playlist extractor.
//!
//! @author jasmith79
//! @license MIT
//! @copyright 2023
mod cli;
mod library_xml;
mod m3u;
mod path_utils;
mod track;

use cli::parse_args;
use library_xml::{get_itunes_prefix, read_xml};
use m3u::to_m3u_playlist;
use path_utils::write_to_file;

fn main() {
    let default_lists = vec!["Downloaded", "Library", "Music"];
    let args = parse_args();
    let library_data = read_xml(&args.path);
    let itunes_prefix = get_itunes_prefix(&library_data);
    for playlist in &library_data.playlists {
        let name: &str = &playlist.name;
        if args.include_default_playlists || !default_lists.contains(&name) {
            if args.verbose > 0 {
                println!("Converting playlist {}.", playlist.name);
            }
            let (name, contents) =
                to_m3u_playlist(&itunes_prefix, &args, &library_data.tracks, playlist);
            let path = args.output_path.join(name);
            write_to_file(&args, &path, &contents);
            if args.verbose > 1 {
                println!("Done.");
            }
        }
    }
}
