//! # Blog Tools
//!
//! `blog-tools` is a collection of tools that helps make blogs in Rust.
//!
//! For this to work, you should have a folder, for instance, `blog` which serves
//! as the root. Within this folder, you can have any folder structure you like,
//! so long as there is a markdown file and a JSON file next to each other. The
//! markdown and JSON file need to have the same file stem. For example, you could
//! have
//!
//! - blog
//!     - 2023
//!         - 2023-01-01
//!             - my_first_blog.json
//!             - my_first_blog.md
//!         - (other folders)
//!
//! The JSON must conform to the following schema
//!
//! ```rust,ignore
//! {
//! "title": String,
//! "date": ISO 8601 Date i.e. YYYY-MM-DD,
//! "desc": Optional<String>,
//! "slug": String,
//! "tags": [String],
//! "keywords": Optional<[String]>,
//! "canonical_link": Optional<String>,
//! "author_name": Optional<String>,
//! "author_webpage": Optional<String>
//! }
//! ```
//!
//! If you put the whole blog inside a lazy static it can massively help with
//! website speed
//!
//! ```rust,ignore
//! pub static BLOG_ROOT: &str = "blog";
//! lazy_static! {
//!    pub static ref STATIC_BLOG_ENTRIES: Blog = get_blog(PathBuf::from(BLOG_ROOT), None, None);
//! }
//! ```
#![warn(missing_docs)]

use markdown::mdast::Node;
use parse::get_blog_entries;
use std::path::PathBuf;

mod find;
mod parse;
mod types;

pub use types::Blog;
pub use types::BlogEntry;

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
pub fn get_blog(
    base: PathBuf,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
) -> Blog {
    return get_blog_entries(base, toc_generation_func, preview_chars);
}
