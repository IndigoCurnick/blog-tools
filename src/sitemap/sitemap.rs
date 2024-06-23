use std::{io::Cursor, path::Path};

use xml::{reader::XmlEvent as ReaderXmlEvent, writer::XmlEvent, EmitterConfig, EventReader};

use crate::{
    common::{parse_blogs, BlogError},
    low::LowBlogEntry,
    types::Blog,
};

use super::types::SitemapOptions;

const DATE_FORMAT: &'static str = "%d-%m-%Y";

/// Use this function in `low` mode to generate a sitemap
///
/// Parameters
///
/// - `blog_root`: Path to the root of the blog e.g. `files/blog`
/// - `url_base`: URL of the website e.g. `www.example.com`
/// - `options`: `SitemapOptions` for configuration
pub fn create_sitemap<T: AsRef<Path>>(
    blog_root: T,
    url_base: &String,
    options: &SitemapOptions,
) -> Result<String, BlogError> {
    let (entries, tags): (Vec<LowBlogEntry>, Vec<String>) = parse_blogs(blog_root, None, None)?;

    return create_sitemap_inner(&entries, Some(&tags), url_base, options);
}

pub fn create_sitemap_inner<T: Blog>(
    entries: &Vec<T>,
    maybe_tags: Option<&Vec<String>>,
    url_base: &String,
    options: &SitemapOptions,
) -> Result<String, BlogError> {
    let mut buffer = Cursor::new(Vec::new());
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&mut buffer);

    writer
        .write(
            XmlEvent::start_element("urlset")
                .attr("xmlns", "http://www.sitemaps.org/schemas/sitemap/0.9"),
        )
        .unwrap();

    let default_priority = options.default_priority;

    // Blog pages
    for blog in entries {
        writer.write(XmlEvent::start_element("url")).unwrap();

        // Location
        writer.write(XmlEvent::start_element("loc")).unwrap();

        let loc = format!(
            "{}/{}/{}",
            url_base,
            options.blog_root_slug,
            blog.get_full_slug()
        );

        writer.write(XmlEvent::characters(&loc)).unwrap();
        writer.write(XmlEvent::end_element()).unwrap();

        // Last Modified
        writer.write(XmlEvent::start_element("lastmod")).unwrap();

        let lastmod = match blog.get_last_modified() {
            Some(x) => x.format(DATE_FORMAT).to_string(),
            None => blog.get_date_listed().format(DATE_FORMAT).to_string(),
        };

        writer.write(XmlEvent::characters(&lastmod)).unwrap();
        writer.write(XmlEvent::end_element()).unwrap();

        // Priority

        writer.write(XmlEvent::start_element("priority")).unwrap();

        let priority = match blog.get_priority() {
            None => default_priority,
            Some(x) => {
                if x > 1.0 || x < 0.0 {
                    panic!("Priority must be between 0.0 and 1.0, got `{}`", x);
                }

                x
            }
        };
        writer
            .write(XmlEvent::characters(&format!("{}", priority)))
            .unwrap();
        writer.write(XmlEvent::end_element()).unwrap();

        writer.write(XmlEvent::end_element()).unwrap(); // Finish <url>
    }

    // Tag pages
    if options.include_tags && maybe_tags.is_some() {
        let tags = maybe_tags.unwrap();
        let current_time = chrono::offset::Utc::now();
        let lastmod = current_time.date_naive().format(&DATE_FORMAT).to_string();

        for tag in tags {
            writer.write(XmlEvent::start_element("url")).unwrap();

            // Location
            writer.write(XmlEvent::start_element("loc")).unwrap();

            let loc = format!("{}/{}/{}", url_base, options.tag_root_slug, tag);

            writer.write(XmlEvent::characters(&loc)).unwrap();
            writer.write(XmlEvent::end_element()).unwrap();

            // Last Modified
            writer.write(XmlEvent::start_element("lastmod")).unwrap();

            writer.write(XmlEvent::characters(&lastmod)).unwrap();
            writer.write(XmlEvent::end_element()).unwrap();

            // Priority

            writer.write(XmlEvent::start_element("priority")).unwrap();

            writer
                .write(XmlEvent::characters(&format!("{}", default_priority)))
                .unwrap();
            writer.write(XmlEvent::end_element()).unwrap();

            writer.write(XmlEvent::end_element()).unwrap(); // Finish <url>
        }
    }

    if let Some(sitemap_base) = &options.sitemap_base {
        let parser = EventReader::from_str(&sitemap_base);

        for e in parser {
            match e {
                Ok(ReaderXmlEvent::StartElement { name, .. }) => {
                    let this_name = name.to_string();
                    if this_name == "urlset" {
                        continue;
                    }
                    writer
                        .write(XmlEvent::start_element(this_name.as_str()))
                        .unwrap();
                }
                Ok(ReaderXmlEvent::Characters(x)) => {
                    writer.write(XmlEvent::characters(&x)).unwrap();
                }
                Ok(ReaderXmlEvent::EndElement { name }) => {
                    if name.to_string() == "urlset" {
                        continue;
                    }
                    writer.write(XmlEvent::end_element()).unwrap();
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    break;
                }
                // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
                _ => {}
            }
        }
    }

    writer.write(XmlEvent::end_element()).unwrap(); // End <urlset>

    return Ok(String::from_utf8(buffer.into_inner()).unwrap());
}
