use std::{path::PathBuf, str::FromStr};

use blog_tools::sitemap::generate_sitemap;

extern crate blog_tools;

#[test]
fn test_sitemap() {
    let path = PathBuf::from_str("examples/blog").unwrap();
    let base = "www.example.com".to_string();
    let sitemap = generate_sitemap(path, &base, None, false);

    println!("{}", sitemap);
}
