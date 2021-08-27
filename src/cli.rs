//! cli
//!
//! Parses the command-line arguments for the playlister application.
//!
//! @author jasmith79
//! @license MIT
//! @copyright 2021
use clap;
use std::path::PathBuf;

/// Definition for the struct returned from parse_args.
pub struct PlaylisterArgs {
    pub path: PathBuf,
    pub output_path: PathBuf,
    pub music_path: String,
    pub verbose: bool,
    pub use_file_url: bool,
}

/// Parses the args from env::args_os.
/// returns an instance of PlaylisterArgs.
pub fn parse_args() -> Box<PlaylisterArgs> {
    let matches = clap::App::new("Playlister")
        .version("0.1.0")
        .author("Jared Smith <jasmith79@gmail.com>")
        .about("Converts iTunes® playlists into universal format")
        .arg(clap::Arg::with_name("file_path")
            .help("Path to the file or directory with your iTunes® playlists.")
            .index(1)
            .required(true))
        .arg(clap::Arg::with_name("music_path")
            .help("Path to the music files on the target device. Defaults to existing path in playlist.")
            .takes_value(true)
            .short("m")
            .long("music-path"))
        .arg(clap::Arg::with_name("output_path")
            .help("Path for output, defaults to input path.")
            .takes_value(true)
            .short("o")
            .long("output-path"))
        // Issue here is that some apps (notably VLC) require a
        // file URL and some apps (e.g. on Android) want raw paths.
        .arg(clap::Arg::with_name("use_file_url")
            .help("Whether to use a raw path or a file URL")
            .long("use-file-url"))
        .arg(clap::Arg::with_name("verbose")
            .help("Verbose console output.")
            .short("v")
            .long("verbose"))
        .get_matches();

    let path = matches
        .value_of("file_path")
        .expect("Must include a path to a Library XML file");

    let output_path = matches.value_of("output_path").unwrap_or(path);
    let music_path = matches.value_of("music_path").unwrap_or("").to_string();
    let verbose = matches.is_present("verbose");
    let use_file_url = matches.is_present("use_file_url");
    let parsed_args = Box::new(PlaylisterArgs {
        path: PathBuf::from(path),
        output_path: PathBuf::from(output_path),
        music_path,
        verbose,
        use_file_url,
    });

    return parsed_args;
}
