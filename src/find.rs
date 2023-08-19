use std::{io, path::PathBuf};

use walkdir::WalkDir;

pub fn get_blog_paths(base: PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let base = PathBuf::from(base);
    if !base.is_dir() {
        panic!("BLOG_ROOT is not a directory!")
    }
    let mut markdown_files: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(base.clone()) {
        let entry = entry?;

        let name = match entry.file_name().to_str() {
            Some(x) => x,
            None => continue,
        };

        if name.contains(".json") {
            continue;
        }

        if !(name.contains(".md") || name.contains(".html")) {
            continue;
        }

        markdown_files.push(PathBuf::from(entry.path()));
    }
    markdown_files.sort();
    markdown_files.reverse();
    Ok(markdown_files)
}
