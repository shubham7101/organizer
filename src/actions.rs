use crate::{ config, utils };
use std::io;
use std::path::Path;
use std::fs;

/// Ensures that the given path is a valid directory.
/// If the path exists but is not a directory, returns an error.
/// If the path does not exist, attempts to create it.
fn ensure_directory<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    let dest_path = path.as_ref();

    if dest_path.try_exists().unwrap_or(false) {
        if !dest_path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Path '{}' exists but is not a directory", dest_path.display()),
            ));
        }
    } else {
        fs::create_dir_all(dest_path)?;
    }

    Ok(())
}

pub trait Action {
    // Displays the proposed action for preview.
    fn display_proposed(&self, files_meta_data: &[utils::FileMetaData]);
    // Executes the action.
    fn execute(&self, files_meta_data: &[utils::FileMetaData]) -> Result<(),Vec<io::Error>>;
}

pub struct MoveAction<'a> {
    config: &'a config::MoveConfig,
}

impl<'a> MoveAction<'a> {
    pub fn new(config: &'a config::MoveConfig) -> Self {
        Self { config }
    }

    fn prepare(&self) -> Result<(), io::Error> {
        ensure_directory(&self.config.destination)
    }
}

impl<'a> Action for MoveAction<'a> {
    fn display_proposed(&self, files_meta_data: &[utils::FileMetaData]) {
        for meta_data in files_meta_data {
            println!("{meta_data:?}");
        }
    }

    fn execute(&self, files_meta_data: &[utils::FileMetaData]) -> Result<(),Vec<io::Error>> {
        let mut errors = vec![];
        if let Err(err) = self.prepare() {
            errors.push(err);
            return Err(errors);
        }

        for meta_data in files_meta_data {
            let dest_path = Path::new(&self.config.destination).join(&meta_data.file_name);
            if !self.config.over_ride && dest_path.try_exists().unwrap_or(false) {
                continue;
            }
            if let Err(err) = fs::rename(&meta_data.path, dest_path) {
                errors.push(err);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
