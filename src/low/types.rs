use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{common::BlogJson, types::Blog};

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

impl Blog for LowBlogEntry {
    fn create<T: AsRef<std::path::Path>>(
        blog: T,
        toc_generation_func: Option<&dyn Fn(&markdown::mdast::Node) -> String>,
        preview_chars: Option<usize>,
    ) -> Result<Self, crate::common::BlogError> {
        todo!();
    }
    fn get_title(&self) -> String {
        todo!()
    }

    fn get_date_listed(&self) -> NaiveDate {
        todo!()
    }

    fn get_description(&self) -> Option<String> {
        todo!()
    }

    fn get_html(&self) -> String {
        todo!()
    }

    fn get_full_slug(&self) -> String {
        todo!()
    }

    fn get_part_slug(&self) -> String {
        todo!()
    }

    fn get_tags(&self) -> Vec<String> {
        todo!()
    }

    fn get_table_of_contents(&self) -> Option<String> {
        todo!()
    }

    fn get_keywords(&self) -> Option<Vec<String>> {
        todo!()
    }

    fn get_canonicle_link(&self) -> Option<String> {
        todo!()
    }

    fn get_author_name(&self) -> Option<String> {
        todo!()
    }

    fn get_author_webpage(&self) -> Option<String> {
        todo!()
    }

    fn get_preview(&self) -> String {
        todo!()
    }

    fn get_last_modified(&self) -> Option<NaiveDate> {
        todo!()
    }

    fn get_priority(&self) -> Option<f64> {
        todo!()
    }
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

// TODO: What to do with this? Maybe we can implement `Blog` also?
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
