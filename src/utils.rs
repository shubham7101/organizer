use std::collections::HashSet;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

#[derive(Debug)]
pub struct FileMetaData {
    pub path: PathBuf,
    pub file_name: String,
    pub extension: Option<String>,
    pub is_dir: bool,
    pub is_file: bool,
    pub is_symlink: bool,
    pub size: u64,
    pub permissions: fs::Permissions,
    pub owner_uid: u32,
    pub owner_gid: u32,
    pub created: Option<u64>,
    pub modified: Option<u64>,
}

impl FileMetaData {
    pub fn from_path<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let path = path.as_ref();
        let metadata = fs::metadata(path)?;
        let file_type = metadata.file_type();

        let absolute_path = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        let file_name = path
            .file_stem()
            .and_then(|os_str| os_str.to_str())
            .map_or_else(|| "".to_string(), String::from);
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(String::from);
        let created = metadata
            .created()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs());

        let modified = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs());

        Ok(Self {
            path: absolute_path,
            file_name,
            extension,
            is_dir: file_type.is_dir(),
            is_file: file_type.is_file(),
            is_symlink: file_type.is_symlink(),
            size: metadata.len(),
            permissions: metadata.permissions(),
            owner_uid: metadata.uid(),
            owner_gid: metadata.gid(),
            created,
            modified,
        })
    }

    pub fn is_executable(&self) -> bool {
        let mode = self.permissions.mode();
        mode & 0o111 != 0
    }

    pub fn is_hidden(&self) -> bool {
        self.file_name.starts_with('.')
    }
}

pub fn shortest_paths(paths: Vec<String>) -> Vec<PathBuf> {
    let mut shortest_paths: HashSet<&str> = HashSet::new();

    for path in &paths {
        let mut shortest_path = path;
        for other_path in &paths {
            if shortest_path.starts_with(other_path) {
                shortest_path = other_path;
            }
        }
        shortest_paths.insert(shortest_path);
    }

    shortest_paths.into_iter().map(PathBuf::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_paths() {
        let paths = vec![
            "/home/downloads/abc".to_string(),
            "/home/downloads".to_string(),
            "/var/logs".to_string(),
            "/var/logs/nginx".to_string(),
            "/home/user/docs".to_string(),
        ];

        let filtered_paths = shortest_paths(paths);

        assert!(!filtered_paths.contains(&PathBuf::from("/home/downloads/abc")));
        assert!(filtered_paths.contains(&PathBuf::from("/home/downloads")));
        assert!(filtered_paths.contains(&PathBuf::from("/var/logs")));
        assert!(!filtered_paths.contains(&PathBuf::from("/var/logs/nginx")));
        assert!(filtered_paths.contains(&PathBuf::from("/home/user/docs")));
    }
}
