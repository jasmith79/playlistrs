//! m3u
//!
//! Contains functions pertaining to the creation of the m3u playlist format.
//!
//! @author jasmith79
//! @license MIT
//! @copyright 2023
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;

use partial_application::partial;
use url::Url;

use crate::cli::PlaylisterArgs;
use crate::library_xml::Playlist;
use crate::track::Track;

fn to_m3u_track<P>(itunes_prefix: &P, args: &PlaylisterArgs, track: &Track) -> Option<String>
where
    P: AsRef<Path> + ?Sized,
{
    let name = track.get_name();
    if args.verbose > 3 {
        println!("Converting track {name}");
    }

    let loc = track.location.as_ref()?;
    // If no music path use the original location, the inner
    // option tracks the success of changing the path so we
    // are sure to surface the None if the operation fails rather
    // than .or-ing back in the original location.
    let path = args
        .music_path
        .as_ref()
        .map(|mpath| {
            loc.strip_prefix(itunes_prefix).map_or_else(
                |err| {
                    if args.verbose > 0 {
                        eprintln!("Track {name} does not match the iTunes prefix");
                    }

                    if args.verbose > 2 {
                        eprintln!("Error: {err}");
                    }

                    None
                },
                |stripped| Some(Cow::Owned(mpath.join(stripped))),
            )
        })
        .or(Some(Some(Cow::Borrowed(loc))))??;

    let location = if args.use_file_url {
        Cow::Owned(Url::from_file_path(path.as_ref()).ok()?.to_string())
    } else {
        Cow::Borrowed(path.to_str()?)
    };

    let artist = track.get_artist();
    let duration = track.duration / 1000; // iTunes uses ms but m3u uses seconds

    Some(format!(
        "#EXTINF:{},{} - {}\n{}",
        duration, artist, name, location
    ))
}

pub fn to_m3u_playlist<P>(
    itunes_prefix: &P,
    args: &PlaylisterArgs,
    tracks: &HashMap<String, Track>,
    playlist: &Playlist,
) -> (String, String)
where
    P: AsRef<Path> + ?Sized,
{
    let process_track = partial!(to_m3u_track, itunes_prefix, args, _);
    let playlist_tracks = playlist
        .playlist_items
        .iter()
        .filter_map(|item| {
            // NOTE: no way I know of to avoid the .to_string(): the
            // track_id in PlaylistItem matches to the track_id in the
            // tracks HashMap but serde will panic if you try to do i32 in
            // the HashMap or String for the PlaylistItem.
            let id = &item.track_id.to_string();
            tracks.get(id).and_then(process_track)
        })
        .collect::<Vec<String>>()
        .join("\n");

    let file_name = [&playlist.name, "m3u"].join(".");
    (
        file_name,
        format!("#EXTM3U\n#name={}\n{}", playlist.name, playlist_tracks),
    )
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;
    use crate::library_xml::PlaylistItem;

    #[test]
    fn test_to_m3u_track_basic() {
        let track = Track {
            name: Some(String::from("Favorite Song")),
            location: Some(PathBuf::from("/foo/bar")),
            duration: 3000,
            artist: Some(String::from("dude")),
            album_artist: None,
            composer: None,
        };

        let args = PlaylisterArgs {
            music_path: Some(PathBuf::from("/baz")),
            verbose: 1,
            use_file_url: false,
            path: PathBuf::from("/some/path"),
            output_path: PathBuf::from("/whatever"),
            include_default_playlists: false,
        };

        let prefix = Path::new("/foo");
        let result = to_m3u_track(&prefix, &args, &track);
        assert_eq!(result, Some(String::from("#EXTINF:3,dude - Favorite Song\n/baz/bar")));
    }

    #[test]
    fn test_to_m3u_track_file_url() {
        let track = Track {
            name: Some(String::from("Favorite Song")),
            location: Some(PathBuf::from("/foo/bar")),
            duration: 3000,
            artist: Some(String::from("dude")),
            album_artist: None,
            composer: None,
        };

        let args = PlaylisterArgs {
            music_path: Some(PathBuf::from("/baz")),
            verbose: 1,
            use_file_url: true,
            path: PathBuf::from("/some/path"),
            output_path: PathBuf::from("/whatever"),
            include_default_playlists: false,
        };

        let prefix = Path::new("/foo");
        let result = to_m3u_track(&prefix, &args, &track);
        assert_eq!(result, Some(String::from("#EXTINF:3,dude - Favorite Song\nfile:///baz/bar")));
    }

    #[test]
    fn test_to_m3u_track_no_location() {
        let track = Track {
            name: Some(String::from("Favorite Song")),
            location: None,
            duration: 3000,
            artist: Some(String::from("dude")),
            album_artist: None,
            composer: None,
        };

        let args = PlaylisterArgs {
            music_path: Some(PathBuf::from("/baz")),
            verbose: 1,
            use_file_url: true,
            path: PathBuf::from("/some/path"),
            output_path: PathBuf::from("/whatever"),
            include_default_playlists: false,
        };

        let prefix = Path::new("/foo");
        let result = to_m3u_track(&prefix, &args, &track);
        assert_eq!(result, None);
    }

    #[test]
    fn test_to_m3u_no_music_path() {
        let track = Track {
            name: Some(String::from("Favorite Song")),
            location: Some(PathBuf::from("/foo/bar")),
            duration: 3000,
            artist: Some(String::from("dude")),
            album_artist: None,
            composer: None,
        };

        let args = PlaylisterArgs {
            music_path: None,
            verbose: 1,
            use_file_url: true,
            path: PathBuf::from("/some/path"),
            output_path: PathBuf::from("/whatever"),
            include_default_playlists: false,
        };

        let prefix = Path::new("/foo");
        let result = to_m3u_track(&prefix, &args, &track);
        assert_eq!(result, Some(String::from("#EXTINF:3,dude - Favorite Song\nfile:///foo/bar")));
    }

    #[test]
    fn test_to_m3u_playlist() {
        let prefix = Path::new("/foo");
        let playlist = Playlist {
            name: String::from("foobar"),
            playlist_items: vec![PlaylistItem { track_id: 1 }, PlaylistItem { track_id: 2 }],
        };

        let args = PlaylisterArgs {
            music_path: Some(PathBuf::from("/baz")),
            verbose: 1,
            use_file_url: true,
            path: PathBuf::from("/some/path"),
            output_path: PathBuf::from("/whatever"),
            include_default_playlists: false,
        };

        let tracks = HashMap::from([
            (String::from("1"), Track {
                name: Some(String::from("Favorite Song")),
                location: Some(PathBuf::from("/foo/bar")),
                duration: 3000,
                artist: Some(String::from("dude")),
                album_artist: None,
                composer: None,
            }),
            (String::from("2"), Track {
                name: Some(String::from("Least Favorite Song")),
                location: None,
                duration: 3000,
                artist: Some(String::from("dude")),
                album_artist: None,
                composer: None,
            })

        ]);

        let expected = String::from("#EXTM3U\n#name=foobar\n#EXTINF:3,dude - Favorite Song\nfile:///baz/bar");
        let result = to_m3u_playlist(&prefix, &args, &tracks, &playlist);
        assert_eq!(result, (String::from("foobar.m3u"), expected));
    }
}
