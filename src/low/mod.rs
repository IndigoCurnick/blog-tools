use std::{fs, path::PathBuf};

use chrono::NaiveDate;
use markdown::{mdast::Node, to_html_with_options, Options};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::{
    common::{get_json_data, get_preview, toc, BlogJson},
    HighBlogEntry,
};

pub fn get_blog_tag_list(base: PathBuf) -> Vec<String> {
    let mut tags = vec![];

    for entry in WalkDir::new(base) {
        let entry = entry.unwrap();

        let path = entry.path();

        let extension = match path.extension() {
            Some(x) => x.to_str().unwrap(),
            None => continue,
        };

        if extension != "json" {
            continue;
        }

        let parent = path.parent().unwrap();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        let md_file_name = file_name.replace(".json", ".md");

        let md_file_path = parent.join(md_file_name);

        if !md_file_path.exists() {
            continue;
        }

        let path = path.to_path_buf();

        let json_text = get_json_data(&path).unwrap();

        for tag in &json_text.tags {
            if !tags.contains(tag) {
                tags.push(tag.clone())
            }
        }
    }

    return tags;
}

pub fn preview_blogs_tagged(
    base: PathBuf,
    tag: String,
    preview_length: Option<usize>,
) -> Vec<PreviewBlogEntry> {
    let mut blogs = vec![];

    for entry in WalkDir::new(base) {
        let entry = entry.unwrap();

        if !entry.file_name().to_str().unwrap().ends_with(".json") {
            continue;
        }

        let path = entry.into_path();
        let json = get_json_data(&path).unwrap();

        if !json.tags.contains(&tag) {
            continue;
        }

        // Great! We've found the blog post!

        let parent = path.parent().unwrap();

        let file_name = path.file_name().unwrap().to_str().unwrap();
        let file_base = file_name.replace(".json", ".md");

        let md_path = parent.join(file_base);

        let md = fs::read_to_string(md_path).unwrap();

        let preview = get_preview(&md, preview_length);

        let blog = PreviewBlogEntry::new(json, preview);

        blogs.push(blog);
    }

    return blogs;
}

pub fn render_blog_post(
    base: PathBuf,
    date: String,
    slug: String,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
) -> Option<LowBlogEntry> {
    let split: Vec<&str> = date.split("-").collect();
    let year = split[0];
    let folder = base.join(format!("{}", year)).join(format!("{}", date));

    for entry in WalkDir::new(folder) {
        let entry = entry.unwrap();

        if !entry.file_name().to_str().unwrap().ends_with(".json") {
            continue;
        }

        let path = entry.into_path();
        let json = get_json_data(&path).unwrap();

        if json.slug != slug {
            continue;
        }

        // Great! We've found the blog post!

        let parent = path.parent().unwrap();

        let file_name = path.file_name().unwrap().to_str().unwrap();
        let file_base = file_name.replace(".json", ".md");

        let md_path = parent.join(file_base);

        let md = fs::read_to_string(md_path).unwrap();

        let html = to_html_with_options(
            &md,
            &Options {
                compile: markdown::CompileOptions {
                    allow_dangerous_html: true,
                    allow_dangerous_protocol: true,

                    ..markdown::CompileOptions::default()
                },
                ..markdown::Options::default()
            },
        )
        .unwrap();

        let toc = toc(&md, toc_generation_func);

        return Some(LowBlogEntry::new(json, html, toc));
    }

    return None;
}

pub fn preview_blogs(
    base: PathBuf,
    num: usize,
    preview_length: Option<usize>,
) -> Vec<PreviewBlogEntry> {
    let mut json_paths = vec![];

    for entry in WalkDir::new(base) {
        let entry = entry.unwrap();

        let path = entry.path();

        let extension = match path.extension() {
            Some(x) => x,
            None => continue,
        };

        if extension == "json" {
            json_paths.push(path.to_path_buf());
        }
    }

    json_paths.sort();

    let mut blogs = vec![];

    for i in 0..num {
        let json = get_json_data(&json_paths[i]).unwrap(); // TODO: there's an out of bounds error here

        let parent = json_paths[i].parent().unwrap();
        let file_name = json_paths[i]
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".json", ".md");

        let md_path = parent.join(file_name);

        let markdown = fs::read_to_string(md_path).unwrap();

        let preview: String = get_preview(&markdown, preview_length);

        let blog_preview = PreviewBlogEntry::new(json, preview);

        blogs.push(blog_preview);
    }

    return blogs;
}

/// An individual blog post
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LowBlogEntry {
    /// Title of the blog post
    pub title: String,
    /// Date published
    pub date: NaiveDate,
    /// Description
    pub desc: Option<String>,
    /// The blog post HTML that would be included into a template
    pub html: String,
    /// The URL slug
    pub slug: String,
    /// `Vec` of tags for this blog
    pub tags: Vec<String>,
    /// Table of contents
    pub toc: Option<String>,
    /// Optional `Vec` of keywords. Intended for SEO in comparison to tags
    pub keywords: Option<Vec<String>>,
    /// Optional canonical link, intended for SEO
    pub canonical_link: Option<String>,
    /// Optional author name
    pub author_name: Option<String>,
    /// Optional URL for the author
    pub author_webpage: Option<String>,
}

impl LowBlogEntry {
    pub(crate) fn new(json: BlogJson, html: String, toc: Option<String>) -> Self {
        return LowBlogEntry {
            title: json.title,
            date: json.date,
            desc: json.desc,
            html: html,
            slug: format!("{}/{}", json.date, json.slug),
            tags: json.tags,
            toc: toc,
            keywords: json.keywords,
            canonical_link: json.canonical_link,
            author_name: json.author_name,
            author_webpage: json.author_webpage,
        };
    }
}

/// An individual blog post
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreviewBlogEntry {
    /// Title of the blog post
    pub title: String,
    /// Date published
    pub date: NaiveDate,
    /// Description
    pub desc: Option<String>,
    /// The URL slug
    pub slug: String,
    /// `Vec` of tags for this blog
    pub tags: Vec<String>,
    /// Optional `Vec` of keywords. Intended for SEO in comparison to tags
    pub keywords: Option<Vec<String>>,
    /// Optional canonical link, intended for SEO
    pub canonical_link: Option<String>,
    /// Optional author name
    pub author_name: Option<String>,
    /// Optional URL for the author
    pub author_webpage: Option<String>,
    /// Preview of the blogpost, useful for showing on index pages
    pub preview: String,
}

impl PreviewBlogEntry {
    pub(crate) fn new(json: BlogJson, preview: String) -> Self {
        return PreviewBlogEntry {
            title: json.title,
            date: json.date,
            desc: json.desc,
            slug: format!("{}/{}", json.date, json.slug),
            tags: json.tags,
            keywords: json.keywords,
            canonical_link: json.canonical_link,
            author_name: json.author_name,
            author_webpage: json.author_webpage,
            preview: preview,
        };
    }
}
