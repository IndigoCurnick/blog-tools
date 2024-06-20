use std::{io::Cursor, path::Path};

use xml::{writer::XmlEvent, EmitterConfig};

use crate::{
    common::{get_blog_paths, get_json_data},
    high::HighBlogEntry,
};

const DATE_FORMAT: &'static str = "%d-%m-%Y";
// N.B. `url_base` SHOULD include everything before the blog slug, but doesn't
// need to include the /
// So you would, for example, do https://sub.example.com/blog which would
// generate https://sub.example.com/blog/my-blog, https://sub.example.com/blog/my-other-blog
pub fn generate_sitemap<T: AsRef<Path>>(
    blog_root: T,
    url_base: &String,
    default_priority: Option<f64>,
    include_tags: bool,
) -> String {
    let blog_paths = get_blog_paths(blog_root).unwrap();

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

    let default_priority = match default_priority {
        None => 0.5,
        Some(x) => {
            if x > 1.0 || x < 0.0 {
                panic!("Priority must be between 0.0 and 1.0, got `{}`", x);
            }

            x
        }
    };

    for blog in blog_paths {
        let blog_json = get_json_data(blog).unwrap();

        writer.write(XmlEvent::start_element("url")).unwrap();

        // Location
        writer.write(XmlEvent::start_element("loc")).unwrap();

        let loc = format!("{}/{}", url_base, blog_json.slug);

        writer.write(XmlEvent::characters(&loc)).unwrap();
        writer.write(XmlEvent::end_element()).unwrap();

        // Last Modified
        writer.write(XmlEvent::start_element("lastmod")).unwrap();

        let lastmod = match blog_json.last_modified {
            Some(x) => x.format(DATE_FORMAT).to_string(),
            None => blog_json.date.format(DATE_FORMAT).to_string(),
        };

        writer.write(XmlEvent::characters(&lastmod)).unwrap();
        writer.write(XmlEvent::end_element()).unwrap();

        // Priority

        writer.write(XmlEvent::start_element("priority")).unwrap();

        let priority = match blog_json.priority {
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

    writer.write(XmlEvent::end_element()).unwrap(); // End <urlset>

    return String::from_utf8(buffer.into_inner()).unwrap();
}

// TODO: We need a way to set the part slug for the blog and tag location
// TODO: Need to be able to pass in an already existing sitemap to append together

pub fn create_sitemap_inner(
    entries: &Vec<HighBlogEntry>,
    maybe_tags: Option<&Vec<String>>,
    url_base: &String,
    default_priority: Option<f64>,
) -> String {
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

    let default_priority = match default_priority {
        None => 0.5,
        Some(x) => {
            if x > 1.0 || x < 0.0 {
                panic!("Priority must be between 0.0 and 1.0, got `{}`", x);
            }

            x
        }
    };

    // Blog pages
    for blog in entries {
        writer.write(XmlEvent::start_element("url")).unwrap();

        // Location
        writer.write(XmlEvent::start_element("loc")).unwrap();

        let loc = format!("{}/{}", url_base, blog.slug); // TODO: I'm pretty sure this will make the slug wrong but it's fine for now

        writer.write(XmlEvent::characters(&loc)).unwrap();
        writer.write(XmlEvent::end_element()).unwrap();

        // Last Modified
        writer.write(XmlEvent::start_element("lastmod")).unwrap();

        let lastmod = match blog.last_modified {
            Some(x) => x.format(DATE_FORMAT).to_string(),
            None => blog.date.format(DATE_FORMAT).to_string(),
        };

        writer.write(XmlEvent::characters(&lastmod)).unwrap();
        writer.write(XmlEvent::end_element()).unwrap();

        // Priority

        writer.write(XmlEvent::start_element("priority")).unwrap();

        let priority = match blog.priority {
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
    if let Some(tags) = maybe_tags {
        let current_time = chrono::offset::Utc::now();
        let lastmod = current_time.date_naive().format(&DATE_FORMAT).to_string();

        for tag in tags {
            writer.write(XmlEvent::start_element("url")).unwrap();

            // Location
            writer.write(XmlEvent::start_element("loc")).unwrap();

            let loc = format!("{}/{}", url_base, tag); // TODO: I'm pretty sure this will make the slug wrong but it's fine for now

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

    writer.write(XmlEvent::end_element()).unwrap(); // End <urlset>

    return String::from_utf8(buffer.into_inner()).unwrap();
}
