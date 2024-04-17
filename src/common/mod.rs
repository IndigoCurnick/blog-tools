mod types;
pub use types::{BlogError, BlogJson};

use markdown::{mdast::Node, to_html, to_mdast, ParseOptions};

use std::{
    fs,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

pub fn get_blog_paths<T: AsRef<Path>>(base: T) -> Result<Vec<PathBuf>, BlogError> {
    let base = base.as_ref();
    if !base.is_dir() {
        panic!("BLOG_ROOT is not a directory!")
    }
    let mut markdown_files: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(base) {
        let entry = match entry {
            Ok(x) => x,
            Err(y) => return Err(BlogError::File(y.into())),
        };

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

pub fn get_json_data<T: AsRef<Path>>(blog: T) -> Result<BlogJson, BlogError> {
    let blog = blog.as_ref();

    let parent_path = match blog.parent() {
        Some(x) => x,
        None => return Err(BlogError::FileNotFound),
    };

    let file_name = match blog.file_name() {
        Some(x) => x,
        None => todo!(),
    };

    let file_str = match file_name.to_str() {
        Some(x) => x,
        None => todo!(),
    };

    let name_split: Vec<&str> = file_str.split(".").collect();

    let n = match name_split.get(0) {
        Some(&x) => x,
        None => todo!(),
    };

    let name = format!("{}.json", n);

    let json_path = parent_path.join(name);
    let json_text = match fs::read_to_string(json_path) {
        Ok(x) => x,
        Err(y) => return Err(BlogError::File(y)),
    };

    let json_data: BlogJson = serde_json::from_str(&json_text).unwrap();
    return Ok(json_data);
}

pub fn get_preview(markdown: &String, preview_chars: Option<usize>) -> String {
    let num_chars = match preview_chars {
        Some(x) => x,
        None => 320,
    };
    let preview_md: String = markdown.chars().take(num_chars).collect();
    return to_html(&preview_md);
}

pub fn toc(
    markdown: &String,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
) -> Result<Option<String>, BlogError> {
    return if let Some(toc_gen) = toc_generation_func {
        let mdast = match to_mdast(&markdown, &ParseOptions::default()) {
            Ok(x) => x,
            Err(y) => return Err(BlogError::Markdown(y)),
        };

        Ok(Some(toc_gen(&mdast)))
    } else {
        Ok(None)
    };
}
