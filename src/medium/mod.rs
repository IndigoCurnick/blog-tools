use std::{collections::HashMap, fs, io, path::PathBuf};

use chrono::{Datelike, NaiveDate};
use markdown::{mdast::Node, to_html_with_options, Options};
use serde::{Deserialize, Serialize};

use crate::{common::BlogJson, HighBlogEntry};

use self::parse::get_blog_entries;

mod parse;

pub fn get_medium_blog(
    base: PathBuf,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
) -> MediumBlog {
    return get_blog_entries(base, toc_generation_func, preview_chars);
}

#[derive(Serialize, Deserialize)]
pub struct MediumBlog {
    /// URL slug to individual blog
    ///
    /// Useful when you have a GET request to /blog/\<slug\>
    pub hash: HashMap<String, MediumBlogEntry>,
    /// `Vec` of blog posts, sorted by date
    ///
    /// Useful when you want to list all blog posts e.g. on an index page
    pub entries: Vec<MediumBlogEntry>,
    /// `Vec` of all unique tags
    ///
    /// Useful when you want to list all tags e.g. on an index page
    pub tags: Vec<String>,
}

/// An individual blog post
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediumBlogEntry {
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
    /// Preview of the blogpost, useful for showing on index pages
    pub preview: String,
    file_name: String,
}

impl MediumBlogEntry {
    pub fn new(json: BlogJson, toc: Option<String>, preview: String, file_name: String) -> Self {
        return MediumBlogEntry {
            title: json.title,
            date: json.date,
            desc: json.desc,
            slug: format!("{}/{}", json.date, json.slug),
            tags: json.tags,
            toc: toc,
            keywords: json.keywords,
            canonical_link: json.canonical_link,
            author_name: json.author_name,
            author_webpage: json.author_webpage,
            preview: preview,
            file_name,
        };
    }

    pub fn render(&self, base: PathBuf) -> Result<HighBlogEntry, io::Error> {
        let year = self.date.year();
        let path = base
            .join(format!("{}", year))
            .join(format!("{}", self.date))
            .join(self.file_name.clone());

        println!("I am looking for file {:?}", path);

        let md = fs::read_to_string(path)?;
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

        let high = HighBlogEntry {
            title: self.title.clone(),
            date: self.date.clone(),
            desc: self.desc.clone(),
            html,
            slug: self.slug.clone(),
            tags: self.tags.clone(),
            toc: self.toc.clone(),
            keywords: self.keywords.clone(),
            canonical_link: self.canonical_link.clone(),
            author_name: self.author_name.clone(),
            author_webpage: self.author_webpage.clone(),
            preview: self.preview.clone(),
        };

        return Ok(high);
    }
}
