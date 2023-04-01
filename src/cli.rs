//! cli
//!
//! Parses the command-line arguments for the playlister application.
//!
//! @author jasmith79
//! @license MIT
//! @copyright 2023
use clap::{Arg, ArgAction, Command};
use std::path::PathBuf;

static MUSIC_PATH_HELP: &'static str =
    "Optional path to music files, this will replace the iTunes®\n\
path to your media files. For example, if you have a music\n\
file on your Mac at \n\
/Users/YourName/Music/Music/media.localized/SomeAlbum/SomeSong.m4a\n\
then this path if present will replace everything before\n\
'SomeAlbum'.";

static FILE_URL_HELP: &'static str =
    "By default Playlister will output plain file paths. However,\n\
some applications like VLC expect/work better with file\n\
urls.";

static DEFAULT_LIST_HELP: &'static str =
    "By default iTunes/Music will create a number of playlists,\n\
like Downloaded and playlister will ignore them unless you\n\
pass this flag.";

static ABOUT: &'static str = "Converts iTunes® playlists into universal format used by\n\
most music players.";

#[derive(Debug)]
pub struct PlaylisterArgs {
    pub path: PathBuf,
    pub output_path: PathBuf,
    pub music_path: Option<PathBuf>,
    pub verbose: u8,
    pub use_file_url: bool,
    pub include_default_playlists: bool,
}

fn arg_parser_factory() -> Command {
    let file_path = Arg::new("file_path")
        .help("The path to your iTunes® library.xml")
        .required(true);

    let music_path = Arg::new("music_path")
        .help(MUSIC_PATH_HELP)
        .short('m')
        .long("music-path");

    let output_path = Arg::new("output_path")
        .help("Path to write playlist files to. Defaults to the xml path")
        .short('o')
        .long("output-path");

    let use_file_url = Arg::new("use_file_url")
        .help(FILE_URL_HELP)
        .long("use-file-url")
        .action(ArgAction::SetTrue);

    let include_default_playlists = Arg::new("include_default_playlists")
        .help(DEFAULT_LIST_HELP)
        .long("include-default-playlists")
        .action(ArgAction::SetTrue);

    let verbose = Arg::new("verbose")
        .help("verbose output.")
        .short('v')
        .long("verbose")
        .action(ArgAction::Count);

    clap::Command::new("Playlister")
        .version("0.2.0")
        .author("Jared Smith <jasmith79@gmail.com>")
        .about(ABOUT)
        .arg(file_path)
        .arg(music_path)
        .arg(output_path)
        .arg(use_file_url)
        .arg(include_default_playlists)
        .arg(verbose)
}

fn parse_and_validate(matcher: Command) -> PlaylisterArgs {
    let args = matcher.get_matches();
    let path = args
        .get_one::<String>("file_path")
        .map(|p| PathBuf::from(p))
        .expect("Must include a path to a Library XML file");

    if !path.is_file() {
        panic!("The supplied path must point to a valid iTunes® xml library file");
    }

    // This shouldn't fail for a valid file...
    let input_dir = path.parent().unwrap().to_path_buf();

    let mpath = args
        .try_get_one::<String>("music_path")
        .ok()
        .flatten()
        .map(|p| PathBuf::from(p));

    let opath = args
        .try_get_one::<String>("output_path")
        .ok()
        .flatten()
        .map(|p| PathBuf::from(p))
        .unwrap_or(input_dir);

    if !opath.is_dir() {
        panic!("Output path must be a valid directory.");
    }

    let verbose = args.get_count("verbose");
    let use_file_url = args.get_flag("use_file_url");
    let include_default_playlists = args.get_flag("include_default_playlists");
    if verbose > 2 {
        println!("Path: {:?}", path);
        println!("Output Path: {:?}", opath);
        println!("Music Path: {:?}", mpath);
        println!("Verbosity: {:?}", verbose);
        println!("Use File URL? {:?}", use_file_url);
    }

    PlaylisterArgs {
        path,
        output_path: opath,
        music_path: mpath,
        verbose,
        use_file_url,
        include_default_playlists,
    }
}

pub fn parse_args() -> PlaylisterArgs {
    parse_and_validate(arg_parser_factory())
}
