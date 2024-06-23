/// Options to configure how the sitemap is generated
///
/// `blog-tools` can generate a sitemap for all of the blogs and tags,
/// but obviously doesn't know about any other pages. However, if you provide
/// this struct with a String representation of an XML sitemap it will
/// automatically add those pages into the sitemap for you
pub struct SitemapOptions {
    /// The default priority to use for a blog in the sitemap if no priority is
    /// provided inside the JSON. If this is not explicityly set then the
    /// default is 0.5
    pub default_priority: f64,
    /// Whether to include tag pages in the sitemap. Set this to `true` if your
    /// website has explicit pages where all the blogs of a certain tag are
    /// present
    pub include_tags: bool,
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
