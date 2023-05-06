//! path_utils
//!
//! Special path handling for playlist music paths.
//!
//! @author jasmith79
//! @license MIT
//! @copyright 2023
use std::error::Error;
use std::fmt;
use std::fs::write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Deserializer};
use unicode_normalization::UnicodeNormalization;
use url::Url;

use crate::cli::PlaylisterArgs;

#[derive(Debug)]
pub struct PrefixError;

impl Error for PrefixError {}

impl fmt::Display for PrefixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Could not determine iTunes prefix from first track location"
        )
    }
}

pub fn generate_itunes_prefix<P>(path: &P) -> Result<&Path, PrefixError>
where
    P: AsRef<Path> + ?Sized,
{
    let original = path.as_ref();
    let mut prefix = original;

    while !prefix.ends_with("Music") {
        prefix = match prefix.parent() {
            Some(p) => p,
            None => return Err(PrefixError),
        };
    }

    Ok(prefix)
}

pub fn deserialize_path<'de, D>(deserializer: D) -> Result<Option<PathBuf>, D::Error>
where
    D: Deserializer<'de>,
{
    let de = String::deserialize(deserializer)?;
    // This is one case where this is actually much easier to follow
    // than the Result <-> Option method-chaining route.
    match Url::parse(&de.nfc().to_string()) {
        Ok(url) => Ok(url.to_file_path().ok()),
        Err(_e) => Ok(None),
    }
}

pub fn deserialize_and_normalize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(String::deserialize(deserializer)
        .ok()
        .map(|de| de.nfc().to_string()))
}

pub fn write_to_file<P, C>(args: &PlaylisterArgs, path: &P, contents: &C)
where
    P: AsRef<Path> + std::fmt::Debug,
    C: AsRef<[u8]>,
{
    if args.verbose > 1 {
        println!("Writing file {:?}", path);
    }

    let res = write(path, contents);
    if res.is_err() && args.verbose > 0 {
        eprintln!("Couldn't write file {:?}", path);
    }

    if args.verbose > 2 {
        println!("Done.");
    }
}
