use std::fs;

use chrono::NaiveDate;
use markdown::{to_html_with_options, Options};
use serde::{Deserialize, Serialize};

use crate::{
    common::{get_json_data, preview::get_preview, toc, BlogError, BlogJson},
    types::Blog,
};

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
    last_modified: Option<NaiveDate>,
    priority: Option<f64>,
    previw_chars: Option<usize>,
}

impl Blog for LowBlogEntry {
    fn create<T: AsRef<std::path::Path>>(
        blog: T,
        toc_generation_func: Option<&dyn Fn(&markdown::mdast::Node) -> String>,
        preview_chars: Option<usize>,
    ) -> Result<Self, crate::common::BlogError> {
        let json = get_json_data(&blog)?;

        let markdown = match fs::read_to_string(blog) {
            Ok(x) => x,
            Err(y) => return Err(BlogError::File(y)),
        };

        let html = match to_html_with_options(
            &markdown,
            &Options {
                compile: markdown::CompileOptions {
                    allow_dangerous_html: true,
                    allow_dangerous_protocol: true,

                    ..markdown::CompileOptions::default()
                },
                ..markdown::Options::default()
            },
        ) {
            Ok(x) => x,
            Err(y) => return Err(BlogError::Markdown(y)),
        };

        let toc = toc(&markdown, toc_generation_func)?;

        return Ok(LowBlogEntry {
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
            last_modified: json.last_modified,
            priority: json.priority,
            previw_chars: preview_chars,
        });
    }
    fn get_title(&self) -> String {
        return self.title.clone();
    }

    fn get_date_listed(&self) -> NaiveDate {
        return self.date.clone();
    }

    fn get_description(&self) -> Option<String> {
        return self.desc.clone();
    }

    fn get_html(&self) -> String {
        return self.html.clone();
    }

    fn get_full_slug(&self) -> String {
        return format!("{}/{}", self.get_date_listed(), self.get_part_slug());
    }

    fn get_part_slug(&self) -> String {
        return self.slug.clone();
    }

    fn get_tags(&self) -> Vec<String> {
        return self.tags.clone();
    }

    fn get_table_of_contents(&self) -> Option<String> {
        return self.toc.clone();
    }

    fn get_keywords(&self) -> Option<Vec<String>> {
        return self.keywords.clone();
    }

    fn get_canonicle_link(&self) -> Option<String> {
        return self.canonical_link.clone();
    }

    fn get_author_name(&self) -> Option<String> {
        return self.author_name.clone();
    }

    fn get_author_webpage(&self) -> Option<String> {
        return self.author_webpage.clone();
    }

    fn get_preview(&self) -> String {
        let preview = get_preview(&self.html, self.previw_chars);

        return preview;
    }

    fn get_last_modified(&self) -> Option<NaiveDate> {
        return self.last_modified.clone();
    }

    fn get_priority(&self) -> Option<f64> {
        return self.priority.clone();
    }
}

impl LowBlogEntry {
    pub(crate) fn new(json: BlogJson, html: String, toc: Option<String>) -> Self {
        return LowBlogEntry {
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
            last_modified: json.last_modified,
            priority: json.priority,
            previw_chars: None,
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
            slug: json.slug,
            tags: json.tags,
            keywords: json.keywords,
            canonical_link: json.canonical_link,
            author_name: json.author_name,
            author_webpage: json.author_webpage,
            preview: preview,
        };
    }
}
