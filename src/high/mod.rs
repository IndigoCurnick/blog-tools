use markdown::mdast::Node;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use crate::BlogEntry;

use self::parse::get_blog_entries;

mod find;
mod parse;

/// Gets the whole `Blog` from the specified path. Useful to combine with lazy
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
///     pub static ref STATIC_BLOG_ENTRIES: Blog = get_blog(PathBuf::from(BLOG_ROOT), None, None);
/// }
/// ```
///
pub fn get_high_blog(
    base: PathBuf,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
) -> HighBlog {
    return get_blog_entries(base, toc_generation_func, preview_chars);
}

/// The main `HighBlog` which stores all relevant information for the blog
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
pub struct HighBlog {
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
