use std::{fs, path::Path};

use markdown::{mdast::Node, to_html_with_options, Options};
use walkdir::WalkDir;

use crate::common::{get_json_data, preview::get_preview, toc, BlogError};

use super::types::{LowBlogEntry, PreviewBlogEntry};

/// Use this function to get a list of all unique tags in your blog
///
/// WARNING: With many blogs this function could become extremely slow -
/// maybe consider caching this? Even though this is the no-cache option, a list
/// of single word strings isn't that large. If even this is too large to fit in
/// memory, you probably need a database rather than this crate
pub fn get_blog_tag_list<T: AsRef<Path>>(base: T) -> Result<Vec<String>, BlogError> {
    let mut tags = vec![];

    for entry in WalkDir::new(base) {
        let entry = entry.unwrap();

        let path = entry.path();

        let extension = match path.extension() {
            Some(x) => x.to_str().unwrap(),
            None => continue,
        };

        if extension != "json" {
            continue;
        }

        let parent = match path.parent() {
            Some(x) => x,
            None => return Err(BlogError::FileNotFound),
        };

        let file_name = match path.file_name() {
            Some(x) => x,
            None => return Err(BlogError::FileNotFound),
        };

        let file_name = match file_name.to_str() {
            Some(x) => x,
            None => return Err(BlogError::FileNotFound),
        };

        let md_file_name = file_name.replace(".json", ".md");

        let md_file_path = parent.join(md_file_name);

        if !md_file_path.exists() {
            continue;
        }

        let json_text = get_json_data(path)?;

        for tag in &json_text.tags {
            if !tags.contains(tag) {
                tags.push(tag.clone())
            }
        }
    }

    return Ok(tags);
}

/// This function will find all of the blogs with the specified tag, so they
/// can be previewed (e.g. on a tag index page). They won't contain the full HTML
/// of the blog, only a preview.
///
/// Control the legnth of the preview with `preview_length`. Default is 320 chars
pub fn preview_blogs_tagged<T: AsRef<Path>>(
    base: T,
    tag: String,
    preview_length: Option<usize>,
) -> Result<Vec<PreviewBlogEntry>, BlogError> {
    let mut blogs = vec![];

    for entry in WalkDir::new(base) {
        let entry = entry.unwrap();

        let f_n = match entry.file_name().to_str() {
            Some(x) => x,
            None => return Err(BlogError::FileNotFound),
        };

        if !f_n.ends_with(".json") {
            continue;
        }

        let path = entry.path();
        let json = get_json_data(path)?;

        if !json.tags.contains(&tag) {
            continue;
        }

        // Great! We've found the blog post!

        let parent = match path.parent() {
            Some(x) => x,
            None => return Err(BlogError::FileNotFound),
        };

        let file_name = match path.file_name() {
            Some(x) => x,
            None => return Err(BlogError::FileNotFound),
        };

        let file_name = match file_name.to_str() {
            Some(x) => x,
            None => return Err(BlogError::FileNotFound),
        };

        let file_base = file_name.replace(".json", ".md");

        let md_path = parent.join(file_base);

        let md = match fs::read_to_string(md_path) {
            Ok(x) => x,
            Err(y) => return Err(BlogError::File(y)),
        };

        // TODO: tbh I'm not very happy with rendering the HTML every single time here
        let html = match to_html_with_options(
            &md,
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

        let preview = get_preview(&html, preview_length);

        let blog = PreviewBlogEntry::new(json, preview);

        blogs.push(blog);
    }

    return Ok(blogs);
}

/// Renders an individual blog post.
///
/// Provide the date of the blog post and the slug (the slug as found in the
/// "slug" field of the JSON).
///
/// Optionally, provide a table of contents generation function
///
/// Returns `None` if the specified blog can not be found
pub fn render_blog_post<T: AsRef<Path>>(
    base: T,
    date: String,
    slug: String,
    toc_generation_func: Option<&dyn Fn(&Node) -> String>,
) -> Result<Option<LowBlogEntry>, BlogError> {
    let base = base.as_ref();
    let split: Vec<&str> = date.split("-").collect();

    let year = match split.get(0) {
        Some(&x) => x,
        None => return Err(BlogError::ImproperDate(date.clone())),
    };

    let folder = base.join(format!("{}", year)).join(format!("{}", date));

    for entry in WalkDir::new(folder) {
        let entry = entry.unwrap();

        let f_n = match entry.file_name().to_str() {
            Some(x) => x,
            None => return Err(BlogError::FileNotFound),
        };

        if !f_n.ends_with(".json") {
            continue;
        }

        let path = entry.path();
        let json = get_json_data(path)?;

        if json.slug != slug {
            continue;
        }

        // Great! We've found the blog post!

        let parent = match path.parent() {
            Some(x) => x,
            None => return Err(BlogError::FileNotFound),
        };

        // TODO: I've done this little two stage file name thing several times - there must be scope for abstracting this into a function
        let file_name = match path.file_name() {
            Some(x) => x,
            None => return Err(BlogError::FileNotFound),
        };

        let file_name = match file_name.to_str() {
            Some(x) => x,
            None => return Err(BlogError::FileNotFound),
        };

        let file_base = file_name.replace(".json", ".md");

        let md_path = parent.join(file_base);

        let md = match fs::read_to_string(md_path) {
            Ok(x) => x,
            Err(y) => return Err(BlogError::File(y)),
        };

        let html = match to_html_with_options(
            &md,
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

        let toc = toc(&md, toc_generation_func)?;

        return Ok(Some(LowBlogEntry::new(json, html, toc)));
    }

    return Ok(None);
}

/// Previews blogs for an index page. Will order from newest to oldest
///
/// `num` controls how many blogs will be in the preview
///
/// `preview_length` is how long each preview will be. Default is 320 chars
pub fn preview_blogs<T: AsRef<Path>>(
    base: T,
    num: usize,
    preview_length: Option<usize>,
) -> Result<Vec<PreviewBlogEntry>, BlogError> {
    let mut json_paths = vec![];

    for entry in WalkDir::new(base) {
        let entry = entry.unwrap();

        let path = entry.path();

        let extension = match path.extension() {
            Some(x) => x,
            None => continue,
        };

        if extension == "json" {
            json_paths.push(path.to_path_buf());
        }
    }

    let mut blogs = vec![];

    for i in 0..num {
        let this_path = match json_paths.get(i) {
            Some(x) => x,
            None => break,
        };

        let json = get_json_data(this_path)?;

        let parent = match json_paths[i].parent() {
            // We can use [i] here since we know this must exist :)
            Some(x) => x,
            None => return Err(BlogError::FileNotFound),
        };

        let file_name = match json_paths[i].file_name() {
            Some(x) => x,
            None => return Err(BlogError::FileNotFound),
        };

        let file_name = match file_name.to_str() {
            Some(x) => x,
            None => return Err(BlogError::FileNotFound),
        };

        let file_name = file_name.replace(".json", ".md");

        let md_path = parent.join(file_name);

        let markdown = match fs::read_to_string(md_path) {
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

        let preview: String = get_preview(&html, preview_length);

        let blog_preview = PreviewBlogEntry::new(json, preview);

        blogs.push(blog_preview);
    }

    blogs.sort_by(|a, b| b.date.cmp(&a.date));

    return Ok(blogs);
}
