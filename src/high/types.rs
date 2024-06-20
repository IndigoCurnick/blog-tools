use std::collections::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{common::BlogJson, medium::MediumBlogEntry, types::Blog};

/// The main `HighBlog` which stores all relevant information for the blog
///
/// `hash` contains a map from the url slug, which is constructed from the
/// "slug" field in the `BlogJson` and the date, to the `BlogEntry`
///
/// `entries` contains a date-sorted (newest first) `Vec` of `BlogEntry`.
/// Note that `entries` and `hash` contain the same information
/// but in different formats for performance reasons
///
/// `tags` is an unsorted `Vec` of all unique tags used in the blog
///
#[derive(Debug, Serialize, Deserialize)]
pub struct HighBlog {
    /// URL slug to individual blog
    ///
    /// Useful when you have a GET request to /blog/\<date\>/\<slug\>
    pub hash: HashMap<String, HighBlogEntry>,
    /// `Vec` of blog posts, sorted by date
    ///
    /// Useful when you want to list all blog posts e.g. on an index page
    pub entries: Vec<HighBlogEntry>,
    /// `Vec` of all unique tags
    ///
    /// Useful when you want to list all tags e.g. on an index page
    pub tags: Vec<String>,
    /// XML Representation of the sitemap
    pub sitemap: String,
}

// TODO: Need a better way to manage the slugs - maybe a getter function and then keep the date and slug private?
/// An individual blog post
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HighBlogEntry {
    /// Title of the blog post
    title: String,
    /// Date published
    date: NaiveDate,
    /// Description
    desc: Option<String>,
    /// The blog post HTML that would be included into a template
    html: String,
    /// The URL slug - this does not include the date i.e. `my-blog` instead of `2024-08-19/my-blog`
    slug: String,
    /// `Vec` of tags for this blog
    tags: Vec<String>,
    /// Table of contents
    toc: Option<String>,
    /// Optional `Vec` of keywords. Intended for SEO in comparison to tags
    keywords: Option<Vec<String>>,
    /// Optional canonical link, intended for SEO
    canonical_link: Option<String>,
    /// Optional author name
    author_name: Option<String>,
    /// Optional URL for the author
    author_webpage: Option<String>,
    /// Preview of the blogpost, useful for showing on index pages
    preview: String,
    /// Optional date of last modification - used for sitemap generation.
    /// Default to date when not present
    last_modified: Option<NaiveDate>,
    /// Optionally used for sitemap - default to 0.5 if not present
    priority: Option<f64>,
}

impl Blog for HighBlogEntry {
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

impl HighBlogEntry {
    pub(crate) fn new(json: BlogJson, html: String, toc: Option<String>, preview: String) -> Self {
        return HighBlogEntry {
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
            preview: preview,
            last_modified: json.last_modified,
            priority: json.priority,
        };
    }

    pub(crate) fn new_from_medium(medium: &MediumBlogEntry, html: String) -> Self {
        return Self {
            title: medium.get_title(),
            date: medium.get_date_listed(),
            desc: medium.get_description(),
            html: html,
            slug: medium.get_part_slug(),
            tags: medium.get_tags(),
            toc: medium.get_table_of_contents(),
            keywords: medium.get_keywords(),
            canonical_link: medium.get_canonicle_link(),
            author_name: medium.get_author_name(),
            author_webpage: medium.get_author_webpage(),
            preview: medium.get_preview(),
            last_modified: medium.get_last_modified(),
            priority: medium.get_priority(),
        };
    }
}
