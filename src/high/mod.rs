use chrono::NaiveDate;
use markdown::mdast::Node;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

use crate::common::{BlogError, BlogJson};

use self::parse::get_blog_entries;

mod parse;

/// Gets the whole `HighBlog` from the specified path. Useful to combine with lazy
/// static for loading times
///
/// The path should be a folder which contains markdown files next to json files
///
/// Add an optional toc generation function if you want a table of contents
///
/// Optionally specify the number of chars for the preview. Default is 320.
///
/// ```rust,ignore
/// pub static BLOG_ROOT: &str = "examples/blog/post";

/// lazy_static! {
///     pub static ref STATIC_BLOG_ENTRIES: Blog = get_blog(PathBuf::from(BLOG_ROOT), None, None).unwrap();
/// }
/// ```
///
pub fn get_high_blog<T: AsRef<Path>>(
    base: T,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
    url: &String,
) -> Result<HighBlog, BlogError> {
    return get_blog_entries(base, toc_generation_func, preview_chars, url);
}

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
    /// Optional date of last modification - used for sitemap generation.
    /// Default to date when not present
    pub last_modified: Option<NaiveDate>,
    /// Optionally used for sitemap - default to 0.5 if not present
    pub priority: Option<f64>,
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
}
