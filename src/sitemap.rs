use std::{io::Cursor, path::Path};

use xml::{reader::XmlEvent as ReaderXmlEvent, writer::XmlEvent, EmitterConfig, EventReader};

use crate::{
    common::{get_blog_paths, get_json_data, parse_blogs, BlogError},
    high::HighBlogEntry,
    low::LowBlogEntry,
    types::Blog,
};

const DATE_FORMAT: &'static str = "%d-%m-%Y";

// TODO: Need to be able to pass in an already existing sitemap to append together

pub struct SitemapOptions {
    pub default_priority: f64, // default 0.5
    pub include_tags: bool,    // default false
    /// This represents the location of the blog in the URL. The default is
    /// `blog`. This would mean the individual blog posts are found at
    /// `www.example.com/blog/2024-05-12/my-blog`. If you set this parameter to
    /// `blog-home` then the sitemap would generate
    /// www.example.com/blog-home/2024-05/12/my-blog`
    pub blog_root_slug: String,
    /// This represents the location of the tag index in the URL. The default is
    /// `blog/tag`, if you set `include_tags` to `true`. If `include_tags` is
    /// `false` (default behaviour), then this is ignored.
    /// For example, if you had a tag called `science` then the
    /// URL would be `www.example.com/blog/tag/science`.
    pub tag_root_slug: String,
    /// Optional `String` representation of an XML sitemap. This function will
    /// automatically merge the records of this sitemap into the sitemap it
    /// generates. Useful if you have a bunch of pages which are not part of the
    /// blog that you'd like in the sitemap
    pub sitemap_base: Option<String>,
}

impl Default for SitemapOptions {
    fn default() -> Self {
        Self {
            default_priority: 0.5, // TODO: Maybe move this value into a constant?
            include_tags: false,
            blog_root_slug: "blog".to_string(), // TODO: Maybe move this value into a constant?
            tag_root_slug: "blog/tag".to_string(),
            sitemap_base: None,
        }
    }
}

// ?: I guess this will be used in the Low module? So, I'll use a LowBlogEntry concrete type for now
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
