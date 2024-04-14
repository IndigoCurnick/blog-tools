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

mod common;
mod high;
mod low;
mod medium;

pub use high::{get_high_blog, HighBlog, HighBlogEntry};
pub use low::{
    get_blog_tag_list, preview_blogs, preview_blogs_tagged, render_blog_post, PreviewBlogEntry,
};
pub use medium::{get_medium_blog, MediumBlog, MediumBlogEntry};
