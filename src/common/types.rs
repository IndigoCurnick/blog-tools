use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The main `Blog` which stores all relevant information for the blog
///
/// `hash` contains a map from the url slug defined in the `BlogJson` to the
/// `BlogEntry`
///
/// `entries` contains a date-sorted `Vec` of `BlogEntry`. Note that `entries`
/// and `hash` contain the same information but in different formats for
/// performance reasons
///
/// `tags` is an unsorted `Vec` of all unique tags used in the blog
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Blog {
    /// URL slug to individual blog
    ///
    /// Useful when you have a GET request to /blog/\<slug\>
    pub hash: HashMap<String, BlogEntry>,
    /// `Vec` of blog posts, sorted by date
    ///
    /// Useful when you want to list all blog posts e.g. on an index page
    pub entries: Vec<BlogEntry>,
    /// `Vec` of all unique tags
    ///
    /// Useful when you want to list all tags e.g. on an index page
    pub tags: Vec<String>,
}

/// An individual blog post
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogEntry {
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
    /// Preview of the blogpost, useful for showing on index pages
    pub preview: String,
}

impl BlogEntry {
    pub(crate) fn new(json: BlogJson, html: String, toc: Option<String>, preview: String) -> Self {
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
