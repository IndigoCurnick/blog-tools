use std::{collections::HashMap, fs, path::Path};

use markdown::{mdast::Node, to_html_with_options, Options};

use crate::common::{get_blog_paths, get_json_data, get_preview, toc};

use super::{HighBlog, HighBlogEntry};

pub fn get_blog_entries<T: AsRef<Path>>(
    base: T,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
) -> HighBlog {
    let blog_paths = get_blog_paths(base).unwrap();

    let mut hashes: HashMap<String, HighBlogEntry> = HashMap::new();
    let mut entries: Vec<HighBlogEntry> = vec![];
    let mut tags: Vec<String> = vec![];

    for blog in blog_paths {
        process_blogs(
            &blog,
            &mut hashes,
            &mut entries,
            &mut tags,
            toc_generation_func,
            preview_chars,
        )
    }

    return HighBlog {
        hash: hashes,
        entries: entries,
        tags: tags,
    };
}

fn process_blogs<T: AsRef<Path>>(
    blog: T,
    hashes: &mut HashMap<String, HighBlogEntry>,
    entries: &mut Vec<HighBlogEntry>,
    tags: &mut Vec<String>,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
) {
    let blog = blog.as_ref();
    let json_data = match get_json_data(blog) {
        Ok(x) => x,
        Err(_y) => return,
    };

    for tag in &json_data.tags {
        if !tags.contains(&tag) {
            tags.push(tag.clone());
        }
    }

    let markdown = fs::read_to_string(blog).unwrap();

    let preview: String = get_preview(&markdown, preview_chars);
    let html = to_html_with_options(
        &markdown,
        &Options {
            compile: markdown::CompileOptions {
                allow_dangerous_html: true,
                allow_dangerous_protocol: true,

                ..markdown::CompileOptions::default()
            },
            ..markdown::Options::default()
        },
    )
    .unwrap();

    let toc = toc(&markdown, toc_generation_func);

    let blog_entry = HighBlogEntry::new(json_data, html, toc, preview);

    hashes.insert(blog_entry.slug.clone(), blog_entry.clone());
    entries.push(blog_entry);
}
