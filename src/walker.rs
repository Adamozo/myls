use std::fs::{read_dir, Metadata};
use std::path::{Path, PathBuf};

use crate::args_parser::Config;
use crate::display::{display_long, display_short, display_tree, DisplayMode};
use crate::filters::{filter_entries, FilterType};
use crate::sort::SortArgument;

// -----------------

pub fn start_walking(config: Config) -> std::io::Result<()> {
    let res = match config.display {
        DisplayMode::Long => non_recursive(&config, display_long),
        DisplayMode::Short => non_recursive(&config, display_short),
        DisplayMode::Tree(depth) => {
            recursive_walk(&config.path, depth, 0, &config.filters, &config.sort_by)
        }
    };

    println!();

    res
}

pub fn non_recursive(
    config: &Config,
    display: fn(path: &Path) -> std::io::Result<()>,
) -> std::io::Result<()> {
    if config.path.is_dir() {
        for (path_buf, _) in process_dir(&config.path, &config.filters, &config.sort_by)? {
            display(&path_buf)?;
        }
    } else {
        display(&config.path)?;
    }

    Ok(())
}

fn recursive_walk(
    path: &Path,
    depth: usize,
    current_depth: usize,
    filters: &Vec<FilterType>,
    sort: &SortArgument,
) -> std::io::Result<()> {
    if depth == current_depth {
        return Ok(());
    }

    if path.is_dir() {
        for (path_buf, _) in process_dir(path, filters, sort)? {
            display_tree(&path_buf, current_depth)?;

            if path_buf.is_dir() {
                recursive_walk(&path_buf, depth, current_depth + 1, filters, sort)?;
            }
        }
    } else {
        display_tree(path, current_depth)?;
    }

    Ok(())
}

fn process_dir(
    path: &Path,
    filters: &[FilterType],
    sort: &SortArgument,
) -> std::io::Result<Vec<(PathBuf, Metadata)>> {
    let entries = read_dir(path)?;
    let filtered = filter_entries(entries, filters.to_owned());
    Ok(sort.sort_entries(filtered))
}
