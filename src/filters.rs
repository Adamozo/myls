use std::fs::{self, DirEntry};
use std::path::PathBuf;

use regex::Regex;

#[derive(Debug, Clone)]
pub enum FilterType {
    MinSize(u64),
    MaxSize(u64),
    IsFile,
    IsDir,
    NoHidden,
    MatchesRegex(Regex),
}

impl FilterType {
    pub fn apply(&self, entry: &DirEntry) -> bool {
        match self {
            FilterType::IsDir => entry.metadata().map_or(false, |m| m.is_dir()),
            FilterType::IsFile => entry.metadata().map_or(false, |m| m.is_file()),
            FilterType::MinSize(min_size) => {
                entry.metadata().map_or(false, |m| m.len() >= *min_size)
            }
            FilterType::MaxSize(max_size) => {
                entry.metadata().map_or(false, |m| m.len() <= *max_size)
            }
            FilterType::NoHidden => !entry
                .file_name()
                .to_str()
                .map_or(false, |s| s.starts_with('.')),
            FilterType::MatchesRegex(regex) => entry
                .file_name()
                .to_str()
                .map_or(false, |name| regex.is_match(name)),
        }
    }
}

pub fn filter_entries(
    dir_entries: fs::ReadDir,
    filters: Vec<FilterType>,
) -> impl Iterator<Item = PathBuf> {
    dir_entries
        .filter_map(Result::ok)
        .filter(move |entry| filters.iter().all(|filter| filter.apply(entry)))
        .map(|entry| entry.path())
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use std::fs;
    use std::path::Path;

    const TEMP_DIR: &str = "./test_directory";

    #[test]
    fn test_filter_type_min_size() {
        let read_dir = fs::read_dir(TEMP_DIR).unwrap();
        let min_size_filter = FilterType::MinSize(101);

        let entries = filter_entries(read_dir, vec![min_size_filter]);
        assert_eq!(entries.count(), 2);
    }

    #[test]
    fn test_filter_type_max_size() {
        let temp_dir = Path::new(TEMP_DIR);
        let read_dir = fs::read_dir(temp_dir.join("subdir1")).unwrap();
        let max_size_filter = FilterType::MaxSize(1);

        let entries = filter_entries(read_dir, vec![max_size_filter]);
        assert_eq!(entries.count(), 1);
    }

    #[test]
    fn test_filter_type_is_file() {
        let temp_dir = Path::new(TEMP_DIR);
        let read_dir = fs::read_dir(temp_dir).unwrap();
        let is_file_filter = FilterType::IsFile;

        let mut entries = filter_entries(read_dir, vec![is_file_filter]);
        assert!(entries.all(|entry| entry.metadata().unwrap().is_file()));
    }

    #[test]
    fn test_filter_type_is_dir() {
        let temp_dir = Path::new(TEMP_DIR);
        let read_dir = fs::read_dir(temp_dir).unwrap();
        let is_dir_filter = FilterType::IsDir;

        let mut entries = filter_entries(read_dir, vec![is_dir_filter]);
        assert!(entries.all(|entry| entry.metadata().unwrap().is_dir()));
    }

    #[test]
    fn test_filter_type_no_hidden() {
        let temp_dir = Path::new(TEMP_DIR);
        let read_dir = fs::read_dir(temp_dir).unwrap();
        let no_hidden_filter = FilterType::NoHidden;

        let mut entries = filter_entries(read_dir, vec![no_hidden_filter]);
        assert!(entries.all(|entry| !entry
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with('.')));
    }

    #[test]
    fn test_filter_type_matches_regex() {
        let temp_dir = Path::new(TEMP_DIR);
        let read_dir = fs::read_dir(temp_dir.join("subdir1")).unwrap();
        let regex = Regex::new(r"\.txt$").unwrap();
        let matches_regex_filter = FilterType::MatchesRegex(regex);

        let entries = filter_entries(read_dir, vec![matches_regex_filter]);
        assert_eq!(entries.count(), 2);
    }
}
