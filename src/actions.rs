use crate::{ config, utils };

pub trait Action {
    // Displays the proposed action for preview.
    fn display_proposed(&self, files_meta_data: &[utils::FileMetaData]);
    // Executes the action.
    fn execute(&self, files_meta_data: &[utils::FileMetaData]);
}

//fn join_filename(base: &PathBuf, path: &Path) -> Option<PathBuf> {
//    path.file_name().map(|filename| base.join(filename))
//}

pub struct MoveAction<'a> {
    config: &'a config::MoveConfig,
}

impl<'a> MoveAction<'a> {
    pub fn new(config: &'a config::MoveConfig) -> Self {
        Self { config }
    }
}

impl<'a> Action for MoveAction<'a> {
    fn display_proposed(&self, files_meta_data: &[utils::FileMetaData]) {
        for meta_data in files_meta_data {
            println!("{meta_data:?}");
        }
    }

    fn execute(&self, _files_meta_data: &[utils::FileMetaData]) {}
}
