use std::{collections::HashMap, fs, path::Path};

use markdown::{mdast::Node, to_html_with_options, Options};

use crate::{
    common::{get_blog_paths, get_json_data, preview::get_preview, toc, BlogError},
    sitemap::create_sitemap_inner,
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
) -> Result<HighBlog, BlogError> {
    return get_blog_entries(base, toc_generation_func, preview_chars, url);
}

fn get_blog_entries<T: AsRef<Path>>(
    base: T,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
    url: &String,
) -> Result<HighBlog, BlogError> {
    let blog_paths = get_blog_paths(base).unwrap();

    let mut hashes: HashMap<String, HighBlogEntry> = HashMap::new();
    let mut entries: Vec<HighBlogEntry> = vec![];
    let mut tags: Vec<String> = vec![];

    for blog in blog_paths {
        let out = process_blogs(&blog, &tags, toc_generation_func, preview_chars)?;

        hashes.insert(out.entry.get_full_slug(), out.entry.clone());
        entries.push(out.entry.clone());
        tags.extend(out.tags);
    }

    entries.sort_by(|a, b| b.get_date_listed().cmp(&a.get_date_listed()));

    let sitemap = create_sitemap_inner(&entries, Some(&tags), url, None);

    return Ok(HighBlog {
        hash: hashes,
        entries: entries,
        tags: tags,
        sitemap: sitemap,
    });
}

fn process_blogs<T: AsRef<Path>>(
    blog: T,
    tags: &Vec<String>,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
) -> Result<OneBlog, BlogError> {
    let blog = blog.as_ref();
    let json_data = get_json_data(blog)?;

    let mut out_tags = vec![];
    for tag in &json_data.tags {
        if !tags.contains(&tag) {
            out_tags.push(tag.clone());
        }
    }

    let markdown = match fs::read_to_string(blog) {
        Ok(x) => x,
        Err(y) => return Err(BlogError::File(y)),
    };

    let html = match to_html_with_options(
        &markdown,
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

    let preview: String = get_preview(&html, preview_chars);

    let toc = toc(&markdown, toc_generation_func)?;

    let blog_entry = HighBlogEntry::new(json_data, html, toc, preview);

    return Ok(OneBlog {
        entry: blog_entry,
        tags: out_tags,
    });
}

struct OneBlog {
    pub entry: HighBlogEntry,
    pub tags: Vec<String>,
}
