pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

use std::{
    collections::HashMap,
    fs::{self},
    path::PathBuf,
};

use chrono::NaiveDate;
use markdown::to_html;
use serde::{Deserialize, Serialize};
use std::io;
use walkdir::WalkDir;

type Slug = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Blog {
    pub hash: HashMap<Slug, BlogEntry>,
    pub entries: Vec<BlogEntry>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogEntry {
    pub title: String,
    pub date: NaiveDate,
    pub desc: String,
    pub html: String,
    pub slug: Slug,
    pub tags: Vec<String>,
}

impl BlogEntry {
    pub fn new(json: BlogJson, html: String) -> Self {
        return BlogEntry {
            title: json.title,
            date: json.date,
            desc: json.desc,
            html: html,
            slug: json.slug,
            tags: json.tags,
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlogJson {
    pub title: String,
    pub date: NaiveDate,
    pub desc: String,
    pub slug: String,
    pub tags: Vec<String>,
}

fn get_blog_paths(base: PathBuf) -> Result<Vec<PathBuf>, io::Error> {
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

pub fn get_blog_entries(base: PathBuf) -> Blog {
    // TODO: Error
    let blog_paths = get_blog_paths(base).unwrap();

    let mut hashes: HashMap<Slug, BlogEntry> = HashMap::new();
    let mut entires: Vec<BlogEntry> = vec![];
    let mut tags: Vec<String> = vec![];

    for blog in blog_paths {
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
        println!("json path is {}", json_path.display());
        let json_text = match fs::read_to_string(json_path) {
            Ok(x) => x,
            Err(_y) => continue,
        };

        let json_data: BlogJson = serde_json::from_str(&json_text).unwrap();

        let these_tags = json_data.tags.clone();

        let html = if name_split[1] == ".html" {
            fs::read_to_string(blog).unwrap()
        } else {
            let markdown = fs::read_to_string(blog).unwrap();
            to_html(&markdown)
        };

        let blog_entry = BlogEntry::new(json_data, html);

        hashes.insert(blog_entry.slug.clone(), blog_entry.clone());
        entires.push(blog_entry);

        for tag in these_tags {
            if !tags.contains(&tag) {
                tags.push(tag);
            }
        }
    }

    return Blog {
        hash: hashes,
        entries: entires,
        tags: tags,
    };
}
