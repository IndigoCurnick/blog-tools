use std::{collections::HashMap, fs, path::Path};

use markdown::{mdast::Node, to_html_with_options, Options};

use crate::common::{get_blog_paths, get_json_data, get_preview, toc, BlogError};

use super::{HighBlog, HighBlogEntry};

pub fn get_blog_entries<T: AsRef<Path>>(
    base: T,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
) -> Result<HighBlog, BlogError> {
    let blog_paths = get_blog_paths(base).unwrap();

    let mut hashes: HashMap<String, HighBlogEntry> = HashMap::new();
    let mut entries: Vec<HighBlogEntry> = vec![];
    let mut tags: Vec<String> = vec![];

    for blog in blog_paths {
        let out = process_blogs(&blog, &tags, toc_generation_func, preview_chars)?;

        hashes.insert(out.entry.slug.clone(), out.entry.clone());
        entries.push(out.entry.clone());
        tags.extend(out.tags);
    }

    entries.sort_by_key(|z| z.date);

    return Ok(HighBlog {
        hash: hashes,
        entries: entries,
        tags: tags,
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

    let preview: String = get_preview(&markdown, preview_chars);
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
