use std::{collections::HashMap, path::Path};

use markdown::mdast::Node;

use crate::{
    common::{parse_blogs, BlogError},
    sitemap::{create_sitemap_inner, SitemapOptions},
    types::Blog,
};

use super::types::{HighBlog, HighBlogEntry};

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
    sitemap_options: &SitemapOptions,
) -> Result<HighBlog, BlogError> {
    return get_blog_entries(
        base,
        toc_generation_func,
        preview_chars,
        url,
        sitemap_options,
    );
}

// TODO: I will make a new function which will handle getting the list of blogs and tags, and I should use that here
// TODO: I also wonder if using the `Blog` trait I can abstract this code even more for Medium and Low?
fn get_blog_entries<T: AsRef<Path>>(
    base: T,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
    url: &String,
    sitemap_options: &SitemapOptions,
) -> Result<HighBlog, BlogError> {
    let mut hashes: HashMap<String, HighBlogEntry> = HashMap::new();

    let (mut entries, tags): (Vec<HighBlogEntry>, Vec<String>) =
        parse_blogs(base, toc_generation_func, preview_chars)?;

    for entry in &entries {
        hashes.insert(entry.get_full_slug(), entry.clone());
    }

    entries.sort_by(|a, b| b.get_date_listed().cmp(&a.get_date_listed()));

    let sitemap = create_sitemap_inner(&entries, Some(&tags), url, sitemap_options)?;

    return Ok(HighBlog {
        hash: hashes,
        entries: entries,
        tags: tags,
        sitemap: sitemap,
    });
}
