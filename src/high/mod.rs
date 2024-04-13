use std::path::PathBuf;

use markdown::mdast::Node;

use crate::Blog;

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
) -> Blog {
    return get_blog_entries(base, toc_generation_func, preview_chars);
}
