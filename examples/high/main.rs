use std::path::PathBuf;

use blog_tools::{get_high_blog, BlogEntry, HighBlog};
use lazy_static::lazy_static;
use rocket::{
    fs::{relative, FileServer},
    response::Redirect,
    Request, Route,
};
use rocket_dyn_templates::Template;

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
    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("blog", get_blog_context());
    Some(Template::render("blog_index", context.into_json()))
}

#[get("/blog/<date>/<slug>")]
fn blog_article(date: String, slug: String) -> Option<Template> {
    let mut context = rocket_dyn_templates::tera::Context::new();
    let all_blogs = get_blog_context();
    let complete_slug = format!("{}/{}", date, slug);
    let this_blog = match all_blogs.hash.get(&complete_slug) {
        Some(x) => x,
        None => return None,
    };
    context.insert("blog", this_blog);
    Some(Template::render("blog", context.into_json()))
}

#[get("/blog/tag/<slug>", rank = 2)]
fn tag_page(slug: String) -> Option<Template> {
    let mut context = rocket_dyn_templates::tera::Context::new();
    let all_blogs = get_blog_context();

    let mut these_blogs: Vec<&BlogEntry> = vec![];

    for blog in &all_blogs.entries {
        if blog.tags.contains(&slug) {
            these_blogs.push(&blog);
        }
    }

    context.insert("blogs", &these_blogs);
    context.insert("tag", &slug);
    Some(Template::render("tags", context.into_json()))
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
    return routes![blog_index, blog_article, tag_page];
}

pub static BLOG_ROOT: &str = "examples/blog";

lazy_static! {
    pub static ref STATIC_BLOG_ENTRIES: HighBlog =
        get_high_blog(PathBuf::from(BLOG_ROOT), None, None);
}

fn get_blog_context() -> &'static HighBlog {
    return &STATIC_BLOG_ENTRIES;
}
