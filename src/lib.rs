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

use std::{collections::HashMap, path::PathBuf};

use chrono::NaiveDate;
use markdown::mdast::Node;
use markdown::{to_html, to_mdast, ParseOptions};
use serde::{Deserialize, Serialize};
use std::{fs, io};
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
    pub desc: Option<String>,
    pub html: String,
    pub slug: Slug,
    pub tags: Vec<String>,
    pub toc: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub canonical_link: Option<String>,
    pub author_name: Option<String>,
    pub author_webpage: Option<String>,
    pub preview: String,
}

impl BlogEntry {
    pub fn new(json: BlogJson, html: String, toc: Option<String>, preview: String) -> Self {
        return BlogEntry {
            title: json.title,
            date: json.date,
            desc: json.desc,
            html: html,
            slug: json.slug,
            tags: json.tags,
            toc: toc,
            keywords: json.keywords,
            canonical_link: json.canonical_link,
            author_name: json.author_name,
            author_webpage: json.author_webpage,
            preview: preview,
        };
    }

    pub fn to_meta(&self) -> BlogMeta {
        return BlogMeta {
            title: self.title.clone(),
            date: self.date.clone(),
            desc: self.desc.clone(),
            keywords: self.keywords.clone(),
            canonical_link: self.canonical_link.clone(),
            author_name: self.author_name.clone(),
            author_webpage: self.author_webpage.clone(),
        };
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogMeta {
    pub title: String,
    pub date: NaiveDate,
    pub desc: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub canonical_link: Option<String>,
    pub author_name: Option<String>,
    pub author_webpage: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlogJson {
    pub title: String,
    pub date: NaiveDate,
    pub desc: Option<String>,
    pub slug: String,
    pub tags: Vec<String>,
    pub keywords: Option<Vec<String>>,
    pub canonical_link: Option<String>,
    pub author_name: Option<String>,
    pub author_webpage: Option<String>,
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

pub fn get_blog_entries(
    base: PathBuf,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
) -> Blog {
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

        let markdown = fs::read_to_string(blog).unwrap();

        let num_chars = match preview_chars {
            Some(x) => x,
            None => 300,
        };
        let preview: String = markdown.chars().take(num_chars).collect();
        let html = to_html(&markdown);

        let toc = if toc_generation_func.is_some() {
            let mdast = to_mdast(&markdown, &ParseOptions::default()).unwrap();

            let toc_fn = toc_generation_func.unwrap();

            Some(toc_fn(&mdast))
        } else {
            None
        };

        let blog_entry = BlogEntry::new(json_data, html, toc, preview);

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
