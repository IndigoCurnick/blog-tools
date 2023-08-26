use std::{collections::HashMap, fs, io, path::PathBuf};

use markdown::{mdast::Node, to_html, to_html_with_options, to_mdast, Options, ParseOptions};

use crate::{
    find::get_blog_paths,
    types::{Blog, BlogEntry, BlogJson},
};

pub fn get_blog_entries(
    base: PathBuf,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
) -> Blog {
    let blog_paths = get_blog_paths(base).unwrap();

    let mut hashes: HashMap<String, BlogEntry> = HashMap::new();
    let mut entries: Vec<BlogEntry> = vec![];
    let mut tags: Vec<String> = vec![];

    for blog in blog_paths {
        foo(
            &blog,
            &mut hashes,
            &mut entries,
            &mut tags,
            toc_generation_func,
            preview_chars,
        )
    }

    return Blog {
        hash: hashes,
        entries: entries,
        tags: tags,
    };
}

fn foo(
    blog: &PathBuf,
    hashes: &mut HashMap<String, BlogEntry>,
    entries: &mut Vec<BlogEntry>,
    tags: &mut Vec<String>,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
) {
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

    let blog_entry = BlogEntry::new(json_data, html, toc, preview);

    hashes.insert(blog_entry.slug.clone(), blog_entry.clone());
    entries.push(blog_entry);
}

fn get_json_data(blog: &PathBuf) -> Result<BlogJson, io::Error> {
    let mut json_path = blog.parent().unwrap().to_path_buf();
    let name_split: Vec<&str> = blog
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split(".")
        .collect();
    let name = format!("{}.json", name_split[0]);

    json_path.push(name);
    let json_text = match fs::read_to_string(json_path) {
        Ok(x) => x,
        Err(y) => return Err(y),
    };

    let json_data: BlogJson = serde_json::from_str(&json_text).unwrap();
    return Ok(json_data);
}

fn get_preview(markdown: &String, preview_chars: Option<usize>) -> String {
    let num_chars = match preview_chars {
        Some(x) => x,
        None => 320,
    };
    let preview_md: String = markdown.chars().take(num_chars).collect();
    return to_html(&preview_md);
}

fn toc(markdown: &String, toc_generation_func: Option<&dyn Fn(&Node) -> String>) -> Option<String> {
    let toc = if toc_generation_func.is_some() {
        let mdast = to_mdast(&markdown, &ParseOptions::default()).unwrap();

        let toc_fn = toc_generation_func.unwrap();

        Some(toc_fn(&mdast))
    } else {
        None
    };

    return toc;
}
