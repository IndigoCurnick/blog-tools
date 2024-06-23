use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use markdown::mdast::Node;

use crate::{
    common::{parse_blogs, BlogError},
    sitemap::{create_sitemap_inner, SitemapOptions},
    Blog,
};

use super::types::{MediumBlog, MediumBlogEntry};

/// Gets the whole `MediumBlog` from the specified path. Useful to combine with lazy
/// static for loading times
///
/// The path should be a folder which contains markdown files next to json files
///
/// Add an optional toc generation function if you want a table of contents
///
/// Optionally specify the number of chars for the preview. Default is 320.
///
/// ```rust,ignore
/// lazy_static! {
///     pub static ref STATIC_BLOG_ENTRIES: MediumBlog =
///         get_medium_blog(PathBuf::from(BLOG_ROOT), None, None).unwrap();
///     }
///
/// let this_blog = match all_blogs.hash.get(&complete_slug) {
///     Some(x) => x,
///     None => return None,
/// };
///
/// context.insert(
///     "blog",
///     &this_blog
///         .render(PathBuf::from_str(BLOG_ROOT).unwrap())
///         .unwrap(),
/// );
/// ```
///
pub fn get_medium_blog(
    base: PathBuf,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
    url: &String,
    sitemap_options: &SitemapOptions,
) -> Result<MediumBlog, BlogError> {
    return get_blog_entries(
        base,
        toc_generation_func,
        preview_chars,
        url,
        sitemap_options,
    );
}

pub fn get_blog_entries<T: AsRef<Path>>(
    base: T,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
    url: &String,
    sitemap_options: &SitemapOptions,
) -> Result<MediumBlog, BlogError> {
    let mut hashes: HashMap<String, MediumBlogEntry> = HashMap::new();

    let (mut entries, tags): (Vec<MediumBlogEntry>, Vec<String>) =
        parse_blogs(base, toc_generation_func, preview_chars)?;

    for entry in &entries {
        hashes.insert(entry.get_full_slug(), entry.clone());
    }

    entries.sort_by(|a, b| b.get_date_listed().cmp(&a.get_date_listed()));

    let sitemap = create_sitemap_inner(&entries, Some(&tags), url, sitemap_options)?;

    return Ok(MediumBlog {
        hash: hashes,
        entries: entries,
        tags: tags,
        sitemap: sitemap,
    });
}
