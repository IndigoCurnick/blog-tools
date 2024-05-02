use std::{collections::HashMap, fs, path::Path};

use markdown::{mdast::Node, to_html_with_options, Options};

use crate::common::{get_blog_paths, get_json_data, preview::get_preview, toc, BlogError};

use super::{MediumBlog, MediumBlogEntry};

pub fn get_blog_entries<T: AsRef<Path>>(
    base: T,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
) -> Result<MediumBlog, BlogError> {
    let blog_paths = get_blog_paths(base).unwrap();

    let mut hashes: HashMap<String, MediumBlogEntry> = HashMap::new();
    let mut entries: Vec<MediumBlogEntry> = vec![];
    let mut tags: Vec<String> = vec![];

    for blog in blog_paths {
        let out = process_blogs(&blog, &tags, toc_generation_func, preview_chars)?;

        hashes.insert(out.blog_entry.slug.clone(), out.blog_entry.clone());
        entries.push(out.blog_entry.clone());
        tags.extend(out.tags);
    }

    entries.sort_by(|a, b| b.date.cmp(&a.date));

    return Ok(MediumBlog {
        hash: hashes,
        entries,
        tags,
    });
}

fn process_blogs<T: AsRef<Path>>(
    blog: T,
    tags: &Vec<String>,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
) -> Result<SingleBlogProcess, BlogError> {
    let blog = blog.as_ref();
    let json_data = get_json_data(blog)?;

    let mut out_tags = vec![];
    for tag in &json_data.tags {
        if !tags.contains(&tag) {
            // I guess technically, we can get repeats if the user puts the same tag in the json list...
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

    let file_name = match blog.file_name() {
        Some(x) => x,
        None => return Err(BlogError::FileNotFound),
    };

    let file_name = match file_name.to_str() {
        Some(x) => x.to_string(),
        None => return Err(BlogError::FileNotFound),
    };

    let blog_entry = MediumBlogEntry::new(json_data, toc, preview, file_name);

    return Ok(SingleBlogProcess {
        blog_entry,
        tags: out_tags,
    });
}

struct SingleBlogProcess {
    pub blog_entry: MediumBlogEntry,
    pub tags: Vec<String>,
}
