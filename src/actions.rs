use std::path::{ Path, PathBuf };
use crate::config::MoveConfig;

pub trait Action {
    // Displays the proposed action for preview.
    fn display_proposed(&self, paths: &[PathBuf]);
    // Executes the action.
    fn execute(&self, paths: &[PathBuf]);
}

fn join_filename(base: &PathBuf, path: &Path) -> Option<PathBuf> {
    path.file_name().map(|filename| base.join(filename))
}

pub struct MoveAction<'a> {
    config: &'a MoveConfig,
}

impl<'a> MoveAction<'a> {
    pub fn new(config: &'a MoveConfig) -> Self {
        Self{ config }
    }
}

impl<'a> Action for MoveAction<'a> {
    fn display_proposed(&self, paths: &[PathBuf]) {
        for path in paths {
            println!("MOVE : {0} => {1}", path.display(), self.config.destination);
        }
    }

    fn execute(&self, paths: &[PathBuf]) {
    }
}
