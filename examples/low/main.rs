use blog_tools::{
    low::{
        get_blog_tag_list, preview_blogs, preview_blogs_tagged, render_blog_post, PreviewBlogEntry,
    },
    sitemap::{create_sitemap, SitemapOptions},
};
use rocket::{
    fs::{relative, FileServer},
    response::{content::RawXml, Redirect},
    Request, Route,
};
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, str::FromStr};

pub static BLOG_ROOT: &str = "examples/blog";

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() {
    let port = 8080_u16;

    let figment = rocket::Config::figment()
        .merge(("port", port))
        .merge(("address", "0.0.0.0"));

    if let Err(e) = rocket::custom(figment)
        .mount("/", FileServer::from(relative!("examples/assets/")))
        .register("/", catchers![not_found, error])
        .attach(Template::fairing())
        // .attach(config)
        .mount("/", get_all_routes())
        // .manage(bucket_info)
        .launch()
        .await
    {
        println!("Did not run. Error: {:?}", e)
    }
}

#[get("/blog")]
fn blog_index() -> Option<Template> {
    // I only use this dummy struct to keep consistency with the other two blog modes
    #[derive(Serialize, Deserialize)]
    struct Blogs {
        entries: Vec<PreviewBlogEntry>,
        tags: Vec<String>,
    }

    let mut context = rocket_dyn_templates::tera::Context::new();

    let preview = preview_blogs(PathBuf::from_str(BLOG_ROOT).unwrap(), 2, None).unwrap();
    let tags = get_blog_tag_list(PathBuf::from_str(BLOG_ROOT).unwrap()).unwrap();
    context.insert(
        "blog",
        &Blogs {
            entries: preview,
            tags,
        },
    );
    Some(Template::render("blog_index", context.into_json()))
}

#[get("/blog/<date>/<slug>", rank = 2)]
fn blog_article(date: String, slug: String) -> Option<Template> {
    let mut context = rocket_dyn_templates::tera::Context::new();
    let blog_post =
        render_blog_post(PathBuf::from_str(BLOG_ROOT).unwrap(), date, slug, None).unwrap();

    context.insert("blog", &blog_post);
    Some(Template::render("blog", context.into_json()))
}

#[get("/blog/tag/<slug>")]
fn tag_page(slug: String) -> Option<Template> {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("tag", &slug);
    let all_blogs =
        preview_blogs_tagged(PathBuf::from_str(BLOG_ROOT).unwrap(), slug, None).unwrap();

    context.insert("blogs", &all_blogs);

    Some(Template::render("tags", context.into_json()))
}

pub static URL: &str = "www.example.xyz";

#[get("/sitemap.xml")]
fn sitemap() -> RawXml<String> {
    let base_sitemap = fs::read_to_string("examples/xml/sitemap.xml").unwrap();
    let sitemap = create_sitemap(
        BLOG_ROOT,
        &URL.to_string(),
        &SitemapOptions {
            include_tags: true,
            sitemap_base: Some(base_sitemap),
            ..Default::default()
        },
    );

    return RawXml(sitemap.unwrap());
}

#[catch(404)]
async fn not_found(req: &Request<'_>) -> Redirect {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("url", req.uri());
    Redirect::to(uri!(blog_index))
}

#[catch(500)]
async fn error(req: &Request<'_>) -> Redirect {
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("url", req.uri());
    Redirect::to(uri!(blog_index))
}

fn get_all_routes() -> Vec<Route> {
    return routes![blog_index, blog_article, tag_page, sitemap];
}
