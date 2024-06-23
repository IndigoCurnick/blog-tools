use std::path::Path;

use chrono::NaiveDate;
use markdown::mdast::Node;

use crate::common::BlogError;

// TODO: give these lifetimes so we don't need to clone
/// Primary trait that describes a single blog post. Any struct which derives
/// this is intended to be converted into a JSON with serde and used in a template.
/// You can of course use any of these other methods for any purpose in the
/// blog
pub trait Blog: Clone {
    /// Create a blog post
    ///
    /// Parameters
    ///
    /// - `blog`: Path to the root of the blog
    ///     For this to work, you should have a folder, for instance, `blog` which serves
    ///     as the root. Within this folder, you must have the following structure
    ///
    ///     - blog
    ///         - 2023
    ///             - 2023-01-01
    ///                 - my_first_blog.json
    ///                 - my_first_blog.md
    ///             - (other folders)
    ///
    /// - `toc_generation_func` - A function which parses a blog and generates
    ///     a table of contents. Optional.
    /// - `preview_chars` - number of chars to be taken in the preview of the
    ///     blog. Default is 320.
    fn create<T: AsRef<Path>>(
        blog: T,
        toc_generation_func: Option<&dyn Fn(&Node) -> String>,
        preview_chars: Option<usize>,
    ) -> Result<Self, BlogError>;
    /// Get the blog title
    fn get_title(&self) -> String;
    /// Get the original publication date
    fn get_date_listed(&self) -> NaiveDate;
    /// Get the SEO description
    fn get_description(&self) -> Option<String>;
    /// Get the HTML of the blog
    fn get_html(&self) -> String;
    /// Get the full slug - this would be e.g. `2024-03-19/my-blog`.
    /// In the JSON, you should NOT include the date in the slug
    fn get_full_slug(&self) -> String;
    /// Get the partial slug of the blog. This would be the `slug` field from the
    /// JSON
    fn get_part_slug(&self) -> String;
    /// Get a list of tags for the blog
    fn get_tags(&self) -> Vec<String>;
    /// Get the table of contents. Only present if a table of contents funciton
    /// was provided
    fn get_table_of_contents(&self) -> Option<String>;
    /// Get keywords
    fn get_keywords(&self) -> Option<Vec<String>>;
    /// Get the canonicle link
    fn get_canonicle_link(&self) -> Option<String>;
    /// Get the author
    fn get_author_name(&self) -> Option<String>; // TODO: Maybe support authors?
    /// Get the author webpage
    fn get_author_webpage(&self) -> Option<String>; // TODO: again, support multiple authors?
    /// Get the blog preview. This is the first few hundred characters of the blog,
    /// useful for an index page
    fn get_preview(&self) -> String;
    /// Get the last modified date, mostly use for sitemaps. This is not the
    /// original publication date
    fn get_last_modified(&self) -> Option<NaiveDate>;
    /// Get the priority for the sitemap
    fn get_priority(&self) -> Option<f64>;
}
