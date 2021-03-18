//! track_record
//!
//! Defines the TrackRecord struct for data extracted from the Library
//! XML file.
//! @author jasmith79
//! @license MIT
//! @copyright 2021
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
