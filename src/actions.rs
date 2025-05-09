use crate::{config, utils};
use std::fs;
use std::io;
use std::path::Path;

pub fn parse(action_cfg: &config::Action) -> Box<dyn Action + '_> {
    return match action_cfg {
        config::Action::Move(move_cfg) => Box::new(MoveAction::new(&move_cfg.destination, move_cfg.over_ride)),
        config::Action::Copy(copy_cfg) => Box::new(CopyAction::new(&copy_cfg.destination, copy_cfg.over_ride)),
        config::Action::Delete => Box::new(DeleteAction::new()),
        _ => todo!(),
    };
}

/// Ensures that the given path is a valid directory.
/// If the path exists but is not a directory, returns an error.
/// If the path does not exist, attempts to create it.
fn ensure_directory<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    let dest_path = path.as_ref();

    if dest_path.try_exists().unwrap_or(false) {
        if !dest_path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Path '{}' exists but is not a directory",
                    dest_path.display()
                ),
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
    fn execute(&self, files_meta_data: &[utils::FileMetaData]) -> Result<(), Vec<io::Error>>;
}

struct MoveAction<'a> {
    destination: &'a Path,
    over_ride: bool,
}

impl<'a> MoveAction<'a> {
    fn new(destination: &'a str, over_ride: bool) -> Self {
        Self { 
            destination: Path::new(destination),
            over_ride 
        }
    }

    fn prepare(&self) -> Result<(), io::Error> {
        ensure_directory(self.destination)
    }
}

impl<'a> Action for MoveAction<'a> {
    fn display_proposed(&self, files_meta_data: &[utils::FileMetaData]) {
        for meta_data in files_meta_data {
            println!("{meta_data:?}");
        }
    }

    fn execute(&self, files_meta_data: &[utils::FileMetaData]) -> Result<(), Vec<io::Error>> {
        let mut errors = vec![];
        if let Err(err) = self.prepare() {
            errors.push(err);
            return Err(errors);
        }

        for meta_data in files_meta_data {
            let dest_path = self.destination.join(&meta_data.file_name);
            if !self.over_ride && dest_path.try_exists().unwrap_or(false) {
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

struct CopyAction<'a> {
    destination: &'a Path,
    over_ride: bool,
}

impl<'a> CopyAction<'a> {
    fn new(destination: &'a str, over_ride: bool) -> Self {
        Self {
            destination: Path::new(destination),
            over_ride: over_ride,
        }
    }

    fn prepare(&self) -> Result<(), io::Error> {
        ensure_directory(self.destination)
    }
}

impl<'a> Action for CopyAction<'a> {
    fn display_proposed(&self, files_meta_data: &[utils::FileMetaData]) {
        for meta_data in files_meta_data {
            println!("{meta_data:?}");
        }
    }

    fn execute(&self, files_meta_data: &[utils::FileMetaData]) -> Result<(), Vec<io::Error>> {
        let mut errors = vec![];
        if let Err(err) = self.prepare() {
            errors.push(err);
            return Err(errors);
        }

        for meta_data in files_meta_data {
            let dest_path = self.destination.join(&meta_data.file_name);
            if !self.over_ride && dest_path.try_exists().unwrap_or(false) {
                continue;
            }
            if let Err(err) = fs::copy(&meta_data.path, dest_path) {
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

struct DeleteAction;

impl DeleteAction {
    fn new() -> Self {
        DeleteAction{}
    }
}

impl Action for DeleteAction {
    fn display_proposed(&self, files_meta_data: &[utils::FileMetaData]) {
        for meta_data in files_meta_data {
            println!("{meta_data:?}");
        }
    }

    fn execute(&self, files_meta_data: &[utils::FileMetaData]) -> Result<(), Vec<io::Error>> {
        let mut errors = vec![];

        for meta_data in files_meta_data {
            if meta_data.is_file {
                if let Err(err) = fs::remove_file(&meta_data.path) {
                    errors.push(err);
                }
            } else if meta_data.is_dir {
                if let Err(err) = fs::remove_dir_all(&meta_data.path) {
                    errors.push(err);
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
