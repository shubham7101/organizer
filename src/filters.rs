use crate::utils;

pub trait Filter {
    fn matches(&self, file_meta_data: &utils::FileMetaData) -> bool;
}

pub struct ExtensionFilter<'a> {
    extensions: &'a Vec<String>,
    negate: bool,
}

impl<'a> ExtensionFilter<'a> {
    pub fn new(extensions: &'a Vec<String>, negate: bool) -> Self {
        Self { extensions, negate }
    }
}

impl<'a> Filter for ExtensionFilter<'a> {
    fn matches(&self, file_meta_data: &utils::FileMetaData) -> bool {
        let file_ext = match &file_meta_data.extension {
            Some(ext) => ext,
            None => return self.negate,
        };

        let is_match = self.extensions.iter().any(|ext| ext == file_ext);

        if self.negate {
            !is_match // Invert the result
        } else {
            is_match
        }
    }
}

pub struct NameFilter<'a> {
    starts_with_filter: Option<&'a Vec<String>>,
    ends_with_filter: Option<&'a Vec<String>>,
    contains_filter: Option<&'a Vec<String>>,
}

impl<'a> NameFilter<'a> {
    pub fn new(
        starts_with_filter: Option<&'a Vec<String>>,
        ends_with_filter: Option<&'a Vec<String>>,
        contains_filter: Option<&'a Vec<String>>,
    ) -> Self {
        Self {
            starts_with_filter,
            ends_with_filter,
            contains_filter,
        }
    }

    fn starts_with(&self, file_name: &str) -> bool {
        if let Some(prefixes) = self.starts_with_filter {
            for prefix in prefixes {
                if file_name.starts_with(prefix) {
                    return true;
                }
            }
            return false;
        }
        true
    }

    fn ends_with(&self, file_name: &str) -> bool {
        if let Some(suffixes) = self.ends_with_filter {
            for suffix in suffixes {
                if file_name.ends_with(suffix) {
                    return true;
                }
            }
            return false;
        }
        true
    }

    fn contains(&self, file_name: &str) -> bool {
        if let Some(substrings) = self.contains_filter {
            for substring in substrings {
                if file_name.contains(substring) {
                    return true;
                }
            }
            return false;
        }
        true
    }
}

impl<'a> Filter for NameFilter<'a> {
    fn matches(&self, file_meta_data: &utils::FileMetaData) -> bool {
        let file_name = file_meta_data.file_name.as_str();
        if self.starts_with(file_name) && self.ends_with(file_name) && self.contains(file_name) {
            return true;
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
        let config = vec!["txt".to_string(), "rs".to_string()];
        let filter = ExtensionFilter::new(&config, false);

        assert!(filter.matches(Path::new("file.txt")));
        assert!(filter.matches(Path::new("code.rs")));
        assert!(!filter.matches(Path::new("image.png")));
        assert!(!filter.matches(Path::new("no_extension")));
    }

    #[test]
    fn test_not_extension_filter_matches() {
        let config = vec!["txt".to_string()];
        let filter = ExtensionFilter::new(&config, true);

        assert!(!filter.matches(Path::new("file.txt")));
        assert!(filter.matches(Path::new("code.rs")));
        assert!(filter.matches(Path::new("image.png")));
        assert!(filter.matches(Path::new("no_extension")));
    }

    //#[test]
    //fn test_name_filter_matches() {
    //    let mut config = NameFilterConfig {
    //        case_sensitive: false,
    //        starts_with: Some(vec!["hello".to_string()]),
    //        ends_with: Some(vec!["world".to_string()]),
    //        contains: Some(vec!["lo_wor".to_string()]),
    //    };
    //    let filter = NameFilter::new(&mut config);
    //    // Case: Matches all conditions
    //    assert!(filter.matches(Path::new("hello_world.txt")));
    //
    //    // Case: Fails due to missing starts_with
    //    assert!(!filter.matches(Path::new("world_world.txt"))); // Doesn't start with "hello"
    //
    //    //Case: Fails due to missing ends_with
    //    assert!(!filter.matches(Path::new("hello_hello.rs"))); // Doesn't end with "world"
    //
    //    // Case: Matches when only contains and ends_with are checked
    //    let mut config = NameFilterConfig {
    //        case_sensitive: false,
    //        starts_with: None,
    //        ends_with: Some(vec!["world".to_string()]),
    //        contains: Some(vec!["lo_wor".to_string()]),
    //    };
    //    let filter = NameFilter::new(&mut config);
    //    assert!(filter.matches(Path::new("something_lo_world.txt"))); // Only contains and ends_with
    //
    //    // Case: No filters, should always match
    //    let mut config = NameFilterConfig {
    //        case_sensitive: false,
    //        starts_with: None,
    //        ends_with: None,
    //        contains: None,
    //    };
    //    let filter = NameFilter::new(&mut config);
    //    assert!(filter.matches(Path::new("random_file.txt"))); // Always matches
    //}
}
