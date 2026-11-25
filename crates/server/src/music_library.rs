//! The music tracks found at startup, by mood.

use std::collections::BTreeMap;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::ambiance::Ambiance;

/// The MP3 files of each mood, read once at startup: adding a track takes a
/// restart.
#[derive(Debug, Clone, Default, Serialize)]
pub struct MusicLibrary {
    /// The music folder, holding one subfolder per mood.
    #[serde(skip)]
    pub dir: PathBuf,
    /// The file names of each mood's tracks, sorted.
    #[serde(flatten)]
    pub tracks: BTreeMap<Ambiance, Vec<String>>,
}

impl MusicLibrary {
    /// Lists the `.mp3` files of each mood's folder in `dir`. A missing or
    /// empty folder is only warned about: that mood then plays the default
    /// music.
    pub fn scan(dir: &Path) -> Self {
        let tracks = Ambiance::ALL
            .into_iter()
            .map(|ambiance| (ambiance, tracks_in(&dir.join(ambiance.folder()))))
            .collect::<BTreeMap<_, _>>();
        for (ambiance, names) in &tracks {
            if names.is_empty() {
                let folder = dir.join(ambiance.folder());
                tracing::warn!(
                    "no music for \"{ambiance}\": {} has no .mp3",
                    folder.display()
                );
            }
        }
        Self {
            dir: dir.to_owned(),
            tracks,
        }
    }

    /// The mood to play for `ambiance`: itself when it has tracks, else the
    /// default one.
    pub fn playable(&self, ambiance: Ambiance) -> Ambiance {
        let has_tracks = self
            .tracks
            .get(&ambiance)
            .is_some_and(|names| !names.is_empty());
        if has_tracks {
            ambiance
        } else {
            Ambiance::Default
        }
    }
}

/// The sorted names of the `.mp3` files in `folder`; none when it cannot be
/// read.
fn tracks_in(folder: &Path) -> Vec<String> {
    let entries = match fs::read_dir(folder) {
        Ok(entries) => entries,
        Err(error) => {
            tracing::warn!(%error, "cannot read the music folder {}", folder.display());
            return Vec::new();
        }
    };
    let mut names = Vec::new();
    for entry in entries {
        match entry {
            Ok(entry) if is_track(&entry) => {
                if let Ok(name) = entry.file_name().into_string() {
                    names.push(name);
                }
            }
            Ok(_) => {}
            Err(error) => tracing::warn!(%error, "cannot list {}", folder.display()),
        }
    }
    names.sort();
    names
}

/// Whether the entry is an `.mp3` file whose name is valid Unicode (the
/// pages send names as text).
fn is_track(entry: &DirEntry) -> bool {
    let mp3 = Path::new(&entry.file_name())
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("mp3"));
    mp3 && entry.file_name().to_str().is_some()
        && entry.file_type().is_ok_and(|kind| kind.is_file())
}
