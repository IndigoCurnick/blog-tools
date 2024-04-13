mod types;

use markdown::{mdast::Node, to_html, to_mdast, ParseOptions};
pub use types::BlogJson;

use std::{fs, io, path::PathBuf};

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

pub fn get_json_data(blog: &PathBuf) -> Result<BlogJson, io::Error> {
    let mut json_path = blog.parent().unwrap().to_path_buf();
    let name_split: Vec<&str> = blog
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split(".")
        .collect();
    let name = format!("{}.json", name_split[0]);

    json_path.push(name);
    let json_text = match fs::read_to_string(json_path) {
        Ok(x) => x,
        Err(y) => return Err(y),
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
) -> Option<String> {
    let toc = if toc_generation_func.is_some() {
        let mdast = to_mdast(&markdown, &ParseOptions::default()).unwrap();

        let toc_fn = toc_generation_func.unwrap();

        Some(toc_fn(&mdast))
    } else {
        None
    };

    return toc;
}
