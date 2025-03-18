use std::path::Path;

pub trait Filter {
    fn matches(&self, path: &Path) -> bool;
}

pub struct ExtensionFilter {
    extensions: Vec<String>,
}

impl ExtensionFilter {
    pub fn new(extensions: Vec<String>) -> Self {
        Self{ extensions }
    }
}

impl Filter for ExtensionFilter {
    fn matches(&self, path: &Path) -> bool {
        let file_ext = match path.extension() {
            Some(ext) => ext.to_str(),
            None =>  return false,
        };
        if let Some(ext_str) = file_ext {
            for ext in &self.extensions {
                if ext_str == ext {
                    return true
                }
            }
        }
        false
    }
}

pub struct NameFilter {
    starts_with: Option<Vec<String>>,
    ends_with: Option<Vec<String>>,
    contains: Option<Vec<String>>,
    case_sensitive: bool,
}

impl NameFilter {
    pub fn new(
        mut starts_with: Option<Vec<String>>,
        mut ends_with: Option<Vec<String>>,
        mut contains: Option<Vec<String>>,
        case_sensitive: bool,
    ) -> Self {
        if !case_sensitive {
            starts_with = Self::to_lowercase(starts_with);
            ends_with = Self::to_lowercase(ends_with);
            contains = Self::to_lowercase(contains);
        }

        Self { starts_with, ends_with, contains, case_sensitive }
    }

    fn to_lowercase(list: Option<Vec<String>>) -> Option<Vec<String>> {
        list.map(|vec| vec.into_iter().map(|s| s.to_lowercase()).collect())
    }

    fn starts_with(&self, file_name: &str) -> bool {
        if let Some(prefixes) = &self.starts_with {
            for prefix in prefixes {
                if file_name.starts_with(prefix) {
                    return true
                }
            }
            return false
        }
        true
    }

    fn ends_with(&self, file_name: &str) -> bool {
        if let Some(suffixes) = &self.ends_with {
            for suffix in suffixes {
                if file_name.ends_with(suffix) {
                    return true
                }
            }
            return false
        }
        true
    }

    fn contains(&self, file_name: &str) -> bool {
        if let Some(substrings) = &self.contains {
            for substring in substrings {
                if file_name.contains(substring) {
                    return true
                }
            }
            return false
        }
        true
    }
}

impl Filter for NameFilter {
    fn matches(&self, path: &Path) -> bool {
        let file_name = path.file_stem().and_then(|os_str| os_str.to_str()).unwrap_or("");

        if self.starts_with(file_name) && self.ends_with(file_name) && self.contains(file_name) {
            return true
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_extension_filter_matches() {
        let filter = ExtensionFilter::new(vec!["txt".to_string(), "rs".to_string()]);

        assert!(filter.matches(Path::new("file.txt")));
        assert!(filter.matches(Path::new("code.rs")));
        assert!(!filter.matches(Path::new("image.png")));
        assert!(!filter.matches(Path::new("no_extension")));
    }

    #[test]
    fn test_name_filter_matches() {
        // Case: Matches all conditions
        let filter = NameFilter::new(
            Some(vec!["hello".to_string()]),
            Some(vec!["world".to_string()]),
            Some(vec!["lo_wor".to_string()]),
            false,
        );
        assert!(filter.matches(Path::new("hello_world.txt")));

        // Case: Fails due to missing starts_with
        let filter = NameFilter::new(
            Some(vec!["hello".to_string()]),
            Some(vec!["world".to_string()]),
            Some(vec!["lo_wor".to_string()]),
            false,
        );
        assert!(!filter.matches(Path::new("world_world.txt"))); // Doesn't start with "hello"

         //Case: Fails due to missing ends_with
        let filter = NameFilter::new(
            Some(vec!["hello".to_string()]),
            Some(vec!["world".to_string()]),
            Some(vec!["lo_wor".to_string()]),
            false,
        );
        assert!(!filter.matches(Path::new("hello_hello.rs"))); // Doesn't end with "world"

        // Case: Matches when only contains and ends_with are checked
        let filter = NameFilter::new(
            None,
            Some(vec!["world".to_string()]),
            Some(vec!["lo_wor".to_string()]),
            false,
        );
        assert!(filter.matches(Path::new("something_lo_world.txt"))); // Only contains and ends_with

        // Case: Case-sensitive match should fail
        let filter = NameFilter::new(
            Some(vec!["Hello".to_string()]),
            Some(vec!["World".to_string()]),
            Some(vec!["Lo_Wor".to_string()]),
            true, // Case sensitive
        );
        assert!(!filter.matches(Path::new("hello_world.txt"))); // Case-sensitive mismatch

        // Case: Case-insensitive match should pass
        let filter = NameFilter::new(
            Some(vec!["Hello".to_string()]),
            Some(vec!["World".to_string()]),
            Some(vec!["Lo_Wor".to_string()]),
            false, // Case insensitive
        );
        assert!(filter.matches(Path::new("hello_world.txt"))); // Case-insensitive match

        // Case: No filters, should always match
        let filter = NameFilter::new(None, None, None, false);
        assert!(filter.matches(Path::new("random_file.txt"))); // Always matches
    }

}
