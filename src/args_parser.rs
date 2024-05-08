use crate::display::DisplayMode;
use crate::filters::FilterType;
use crate::sort::SortArgument;
use clap::{Parser, ValueHint};
use regex::Regex;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "ls alternative", long_about = None)]
pub struct Args {
    #[arg(value_name = "PATH", default_value = ".", value_hint = ValueHint::AnyPath)]
    pub path: PathBuf,

    #[arg(
        long,
        value_name = "BYTES",
        help = "Filters files that are at least a certain size in bytes"
    )]
    min: Option<u64>,

    #[arg(
        long,
        value_name = "BYTES",
        help = "Filters files that are at most a certain size in bytes"
    )]
    max: Option<u64>,

    #[arg(short, long, help = "Filters entries that match with given regex")]
    reg: Option<Regex>,

    #[arg(
        short,
        long,
        name = "no-hidden",
        help = "Excludes hidden files from the results",
        default_value = "false"
    )]
    no_hidden: bool,

    #[arg(
        short,
        long,
        help = "Display results in long format",
        conflicts_with = "depth",
        default_value = "false"
    )]
    long: bool,

    #[arg(
        short,
        long,
        help = "Display only files",
        conflicts_with = "directories",
        default_value = "false"
    )]
    files: bool,

    #[arg(
        short,
        long,
        help = "Display only directories",
        conflicts_with = "files",
        default_value = "false"
    )]
    directories: bool,

    #[arg(
        long,
        value_name = "BYTES",
        help = "Display tree structure with given depth"
    )]
    depth: Option<usize>,

    #[arg(short, long, value_name = "SORT", help = "Select sorting method", default_value_t = SortArgument::Name)]
    sort: SortArgument,
}

#[derive(Debug)]
pub struct Config {
    pub path: PathBuf,
    pub filters: Vec<FilterType>,
    pub sort_by: SortArgument,
    pub display: DisplayMode,
}

impl Config {
    pub fn new_from_args(args: Args) -> Self {
        let mut filters: Vec<FilterType> = Vec::new();

        if args.no_hidden {
            filters.push(FilterType::NoHidden);
        }

        if let Some(max) = args.max {
            filters.push(FilterType::MaxSize(max));
        }

        if let Some(min) = args.min {
            filters.push(FilterType::MinSize(min));
        }

        if args.files {
            filters.push(FilterType::IsFile);
        }

        if args.directories {
            filters.push(FilterType::IsDir);
        }

        if let Some(regex) = args.reg {
            filters.push(FilterType::MatchesRegex(regex));
        }

        // -----------------------------------------------

        let display = {
            if let Some(depth) = args.depth {
                DisplayMode::Tree(depth)
            } else if args.long {
                DisplayMode::Long
            } else {
                DisplayMode::Short
            }
        };

        // -----------------------------------------------

        Self {
            path: args.path,
            filters,
            sort_by: args.sort,
            display,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_default_arguments() {
        let args = Args::parse_from(["testapp"]);
        assert_eq!(args.path.to_str().unwrap(), ".");
        assert!(!args.long);
        assert_eq!(args.sort, SortArgument::Name);
        assert!(!args.no_hidden);
    }

    #[test]
    fn test_custom_path_and_filters() {
        let args = Args::parse_from([
            "testapp",
            "/some/path",
            "--min",
            "100",
            "--max",
            "200",
            "--no-hidden",
        ]);
        assert_eq!(args.path.to_str().unwrap(), "/some/path");
        assert_eq!(args.min.unwrap(), 100);
        assert_eq!(args.max.unwrap(), 200);
        assert!(args.no_hidden);
    }

    #[test]
    fn test_conflicting_args() {
        let result = Args::try_parse_from(["testapp", "--files", "--directories"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_initialization() {
        let args = Args::parse_from(["testapp", "--sort", "size"]);
        let config = Config::new_from_args(args);
        assert_eq!(config.sort_by, SortArgument::Size);
    }

    #[test]
    fn test_regex_argument() {
        let args = Args::parse_from(["testapp", "--reg", r"test"]);
        assert!(args.reg.is_some());
        assert!(args.reg.unwrap().is_match("this is a test"));
    }

    #[test]
    fn test_depth_argument() {
        let args = Args::parse_from(["testapp", "--depth", "3"]);
        assert_eq!(args.depth, Some(3));
    }

    #[test]
    fn test_display_mode_with_depth() {
        let args = Args::parse_from(["testapp", "--depth", "2"]);
        let config = Config::new_from_args(args);
        if let DisplayMode::Tree(depth) = config.display {
            assert_eq!(depth, 2);
        } else {
            panic!("Display mode should be set to Tree with specified depth");
        }
    }

    #[test]
    fn test_no_hidden_with_depth() {
        let args = Args::parse_from(["testapp", "--no-hidden", "--depth", "5"]);
        let config = Config::new_from_args(args);

        match config.display {
            DisplayMode::Tree(depth) => assert_eq!(depth, 5),
            _ => panic!("Display mode should be Tree when depth is specified"),
        }
    }

    #[test]
    fn test_conflict_files_and_directories() {
        let result = Args::try_parse_from(["testapp", "--files", "--directories"]);
        assert!(
            result.is_err(),
            "Parsing should fail when conflicting arguments are provided"
        );
    }
}
