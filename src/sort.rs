use clap::Parser;
use std::fmt;
use std::fs::Metadata;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Parser, Debug, Clone, PartialEq)]
pub enum SortArgument {
    Date,
    Size,
    Name,
}

impl fmt::Display for SortArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for SortArgument {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "date" => Ok(SortArgument::Date),
            "size" => Ok(SortArgument::Size),
            "name" => Ok(SortArgument::Name),
            _ => Err(format!("'{}' is not a valid value for SortArgument", s)),
        }
    }
}

impl SortArgument {
    pub fn sort_entries(&self, entries: impl Iterator<Item = PathBuf>) -> Vec<(PathBuf, Metadata)> {
        let mut entries_vec: Vec<_> = entries
            .filter_map(|entry| {
                let metadata = entry.metadata().ok()?;
                Some((entry.to_path_buf(), metadata))
            })
            .collect();

        match self {
            SortArgument::Date => entries_vec.sort_by_key(|(_, metadata)| {
                metadata
                    .modified()
                    .unwrap_or_else(|_| std::time::SystemTime::now())
            }),
            SortArgument::Size => entries_vec.sort_by_key(|(_, metadata)| metadata.len()),
            SortArgument::Name => {
                entries_vec.sort_by_key(|(path, _)| path.file_name().unwrap().to_os_string())
            }
        }
        entries_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_from_str_valid() {
        assert_eq!(SortArgument::from_str("date"), Ok(SortArgument::Date));
        assert_eq!(SortArgument::from_str("size"), Ok(SortArgument::Size));
        assert_eq!(SortArgument::from_str("name"), Ok(SortArgument::Name));
    }

    #[test]
    fn test_from_str_invalid() {
        assert!(SortArgument::from_str("unknown").is_err());
    }

    fn create_test_file(dir: &PathBuf, file_name: &str, size: u64) -> PathBuf {
        let file_path = dir.join(file_name);
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{:0<width$}", "", width = size as usize).unwrap();
        file.sync_all().unwrap();
        file_path
    }

    #[test]
    fn test_sort_entries_by_name() {
        let temp_dir = tempdir().unwrap();
        let dir = temp_dir.path().to_path_buf();
        create_test_file(&dir, "b.txt", 100);
        create_test_file(&dir, "a.txt", 200);
        create_test_file(&dir, "c.txt", 300);

        let sort_arg = SortArgument::Name;
        let entries = fs::read_dir(&dir).unwrap().map(|e| e.unwrap().path());
        let sorted = sort_arg.sort_entries(entries);

        assert_eq!(sorted[0].0.file_name().unwrap().to_str().unwrap(), "a.txt");
        assert_eq!(sorted[1].0.file_name().unwrap().to_str().unwrap(), "b.txt");
        assert_eq!(sorted[2].0.file_name().unwrap().to_str().unwrap(), "c.txt");
    }

    #[test]
    fn test_sort_entries_by_size() {
        let temp_dir = tempdir().unwrap();
        let dir = temp_dir.path().to_path_buf();
        create_test_file(&dir, "file1.txt", 300);
        create_test_file(&dir, "file2.txt", 100);
        create_test_file(&dir, "file3.txt", 200);

        let sort_arg = SortArgument::Size;
        let entries = fs::read_dir(&dir).unwrap().map(|e| e.unwrap().path());
        let sorted = sort_arg.sort_entries(entries);

        assert_eq!(
            sorted[0].0.file_name().unwrap().to_str().unwrap(),
            "file2.txt"
        );
        assert_eq!(
            sorted[1].0.file_name().unwrap().to_str().unwrap(),
            "file3.txt"
        );
        assert_eq!(
            sorted[2].0.file_name().unwrap().to_str().unwrap(),
            "file1.txt"
        );
    }
}
