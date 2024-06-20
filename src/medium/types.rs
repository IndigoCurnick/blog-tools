use std::{collections::HashMap, fs, path::PathBuf};

use chrono::{Datelike, NaiveDate};
use markdown::{to_html_with_options, Options};
use serde::{Deserialize, Serialize};

use crate::{
    common::{BlogError, BlogJson},
    high::HighBlogEntry,
    types::Blog,
};

/// The main `MediumBlog` which stores all relevant information for the blog
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
#[derive(Serialize, Deserialize)]
pub struct MediumBlog {
    /// URL slug to individual blog
    ///
    /// Useful when you have a GET request to /blog/\<date\>/\<slug\>
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

/// An individual blog post. You will need to render this using `render`
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
    file_name: String, // ! can't be present in `high` or `low`
    pub last_modified: Option<NaiveDate>,
    pub priority: Option<f64>,
}

impl Blog for MediumBlogEntry {
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

impl MediumBlogEntry {
    pub(crate) fn new(
        json: BlogJson,
        toc: Option<String>,
        preview: String,
        file_name: String,
    ) -> Self {
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
            last_modified: json.last_modified,
            priority: json.priority,
        };
    }

    /// Use this function to render a `MediumBlogEntry` into a `HighBlogEntry`,
    /// which then contains the full blog HTML you can return to a user
    pub fn render(&self, base: PathBuf) -> Result<HighBlogEntry, BlogError> {
        let year = self.date.year();
        let path = base
            .join(format!("{}", year))
            .join(format!("{}", self.date))
            .join(self.file_name.clone());

        let md = match fs::read_to_string(path) {
            Ok(x) => x,
            Err(y) => return Err(BlogError::File(y)),
        };

        let html = match to_html_with_options(
            &md,
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

        let high = HighBlogEntry::new_from_medium(self, html);

        return Ok(high);
    }
}
