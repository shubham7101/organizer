use std::path::{ Path, PathBuf };
use std::fs;

pub trait Action {
    // Displays the proposed action for preview.
    fn display_proposed(&self, path: &Path);
    // Executes the action.
    fn execute(&self, path: &Path);
}

fn join_filename(base: &PathBuf, path: &Path) -> Option<PathBuf> {
    path.file_name().map(|filename| base.join(filename))
}

pub struct MoveAction {
    destination: PathBuf,
}

impl MoveAction {
    pub fn new(path: &str) -> Self {
        let path = PathBuf::from(path);
        Self{ destination: path }
    }
}

impl Action for MoveAction {
    fn display_proposed(&self, path: &Path) {
        println!("MOVE : {0} => {1}", path.display(), self.destination.display());
    }

    fn execute(&self, path: &Path) {
        if let Some(final_dest) = join_filename(&self.destination, path) {
            if let Err(e) = fs::rename(path, &final_dest) {
                eprintln!("❌ Failed to move {} to {}: {}", path.display(), final_dest.display(), e);
            } else {
                println!("✅ Successfully moved {} to {}", path.display(), final_dest.display());
            }
        } else {
            eprintln!("❌ Cannot move {}: No filename found!", path.display());
        }
    }
}
