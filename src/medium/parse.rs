use std::{collections::HashMap, fs, path::PathBuf};

use markdown::mdast::Node;

use crate::common::{get_blog_paths, get_json_data, get_preview, toc};

use super::{MediumBlog, MediumBlogEntry};

pub fn get_blog_entries(
    base: PathBuf,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
    preview_chars: Option<usize>,
) -> MediumBlog {
    let blog_paths = get_blog_paths(base).unwrap();

    let mut hashes: HashMap<String, MediumBlogEntry> = HashMap::new();
    let mut entries: Vec<MediumBlogEntry> = vec![];
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

    return MediumBlog {
        hash: hashes,
        entries: entries,
        tags: tags,
    };
}

fn process_blogs(
    blog: &PathBuf,
    hashes: &mut HashMap<String, MediumBlogEntry>,
    entries: &mut Vec<MediumBlogEntry>,
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

    let toc = toc(&markdown, toc_generation_func);

    let file_name = blog.file_name().unwrap().to_str().unwrap().to_string();

    let blog_entry = MediumBlogEntry::new(json_data, toc, preview, file_name);

    hashes.insert(blog_entry.slug.clone(), blog_entry.clone());
    entries.push(blog_entry);
}
