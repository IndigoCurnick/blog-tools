pub mod preview;
mod types;

use markdown::{mdast::Node, to_mdast, ParseOptions};
pub use types::{BlogError, BlogJson};

use std::{
    fs,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

use crate::Blog;

// TODO: Better name?
pub fn parse_blogs<T: AsRef<Path>, U: Blog>(
    base: T,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
) -> Result<(Vec<U>, Vec<String>), BlogError> {
    let blog_paths = get_blog_paths(base)?;

    let mut entries = vec![];
    let mut tags = vec![]; // TODO: would it be worth converting tags into a Set<String>?

    for blog_path in blog_paths {
        let out: U = process_blogs(blog_path, toc_generation_func, preview_chars)?;
        entries.push(out.clone());

        for tag in &out.get_tags() {
            if !tags.contains(tag) {
                tags.push(tag.clone());
            }
        }
    }

    return Ok((entries, tags));
}

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
        None => return Err(BlogError::FileNotFound),
    };

    let file_str = match file_name.to_str() {
        Some(x) => x,
        None => return Err(BlogError::FileNotFound),
    };

    let name_split: Vec<&str> = file_str.split(".").collect();

    let n = match name_split.get(0) {
        Some(&x) => x,
        None => return Err(BlogError::ImproperFileName(file_str.to_string())),
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

fn process_blogs<T: AsRef<Path>, U: Blog>(
    blog: T,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
) -> Result<U, BlogError> {
    return U::create(blog, toc_generation_func, preview_chars);
}
