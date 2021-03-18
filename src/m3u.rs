//! m3u
//!
//! Contains functions pertaining to the creation of the m3u playlist format.
//!
//! @author jasmith79
//! @license MIT
//! @copyright 2021
use crate::track_record::TrackRecord;
use percent_encoding::percent_decode_str;

/// Converts the vector of TrackRecords into an m3u String.
pub fn convert(title: &str, tracks: &Vec<TrackRecord>) -> String {
    let converted = tracks
        .iter()
        .map(|track| {
            let duration = track.duration_ms / 1000;
            // Artist for the playlist is determined by the following
            // priority, highest present in the record wins:
            // Artist > Album Artist -> Composer
            let artist_ref = if !track.artist.is_empty() {
                &track.artist
            } else if !track.album_artist.is_empty() {
                &track.album_artist
            } else if !track.composer.is_empty() {
                &track.composer
            } else {
                ""
            };

            let artist = percent_decode_str(artist_ref)
                .decode_utf8()
                .expect("Artist is present in the record but not a UTF-8 encoded string.")
                .to_string();

            let name = percent_decode_str(&track.name)
                .decode_utf8()
                .expect("Track Name is present in the record but not a UTF-8 encoded string.")
                .to_string();

            return format!(
                "#EXTINF:{},{} - {}\n{}",
                duration, artist, name, track.location,
            );
        })
        .collect::<Vec<String>>()
        .join("\n");

    return format!("#EXTM3U\n#name={}\n{}", title, converted);
}
