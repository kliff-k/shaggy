use std::path::{Path, PathBuf};
use rand::prelude::IndexedRandom;
use walkdir::WalkDir;
use crate::shared::types::Error;

pub async fn get_random_song(dir_path: impl AsRef<Path>) -> Result<Option<PathBuf>, Error> {
    let mut songs = Vec::new();
    let dir_path = dir_path.as_ref();

    if !dir_path.is_dir() {
        return Ok(None);
    }

    for entry in WalkDir::new(dir_path).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "mp3" || ext == "ogg" || ext == "wav" {
                    songs.push(path.to_path_buf());
                }
            }
        }
    }

    if songs.is_empty() {
        Ok(None)
    } else {
        let mut rng = rand::rng();
        Ok(songs.choose(&mut rng).cloned())
    }
}