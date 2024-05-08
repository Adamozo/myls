use chrono::offset::Local;
use chrono::DateTime;
use std::fs::Metadata;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

#[derive(Debug)]
pub enum DisplayMode {
    Long,
    Short,
    Tree(usize),
}

pub fn display_long(path: &Path) -> std::io::Result<()> {
    let metadata = path.metadata()?;

    let size = metadata.len();

    let permissions = metadata.permissions();
    let mode = permissions.mode();

    let modified_time = metadata.modified()?;
    let datetime: DateTime<Local> = DateTime::from(modified_time);
    let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    println!(
        "{} | {:>4o} | {:>10}  | {} | {:<20}",
        format_type(&metadata),
        mode & 0o777,
        size,
        formatted_time,
        path.file_name()
            .unwrap()
            .to_str()
            .unwrap_or("!!! Invalid path !!!")
    );
    Ok(())
}

pub fn display_short(path: &Path) -> std::io::Result<()> {
    print!(
        "{} {}  ",
        format_type(&path.metadata()?),
        path.file_name()
            .unwrap()
            .to_str()
            .unwrap_or("!!! Invalid path !!!")
    );
    Ok(())
}

pub fn display_tree(path: &Path, current_depth: usize) -> std::io::Result<()> {
    let metadata = path.metadata()?;

    println!(
        "├──{}{} {}",
        "──".repeat(current_depth),
        format_type(&metadata),
        path.file_name()
            .unwrap()
            .to_str()
            .unwrap_or("!!! Invalid path !!!")
    );
    Ok(())
}

fn format_type(metadata: &Metadata) -> &'static str {
    if metadata.is_file() {
        "📄"
    } else if metadata.is_dir() {
        "📁"
    } else if metadata.is_symlink() {
        "🔗"
    } else {
        "❓"
    }
}
