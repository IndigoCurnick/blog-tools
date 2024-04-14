# Blog Tools

`blog-tools` is a collection of tools that helps make blogs in Rust.

For this to work, you should have a folder, for instance, `blog` which serves
as the root. Within this folder, you must have the following structure

- blog
    - 2023
        - 2023-01-01
            - my_first_blog.json
            - my_first_blog.md
        - (other folders)

That is, organised by year, organised by date (yyyy-mm-dd), then a `blog.md`
next to a `blog.json`.

The JSON must conform to the following schema

```rust,ignore
{
"title": String,
"date": ISO 8601 Date i.e. YYYY-MM-DD,
"desc": Optional<String>,
"slug": String,
"tags": [String],
"keywords": Optional<[String]>,
"canonical_link": Optional<String>,
"author_name": Optional<String>,
"author_webpage": Optional<String>
}
```

## Slugs

In `blog-tools` all slugs are /{date}/{sub-slug}.

Make sure the "slug" filed in the JSON is *just* the final sub-slug
- `blog-tools` will automatically handle the date for you

## How This Crate is Organised

There's three modules of interest

- `high`
- `medium`
- `low`

These refer to the expected RAM usage of each of these systems at runtime.
Please select the style which suits the number of blogs you have the most.
Do note that this crate is not intended to handle literally millions of

`high` uses the most RAM by storing the entire blog in memory the whole time,
the best way to do this is using a lazy static like so

```rust,ignore
lazy_static! {
    pub static ref STATIC_BLOG_ENTRIES: HighBlog =
        get_high_blog(PathBuf::from(BLOG_ROOT), None, None);
    }
```

`medium` stores the majority of the blog, but not the rendered HTML of the
blog posts themselves. These will need to be rendered when requested

```rust,ignore
lazy_static! {
    pub static ref STATIC_BLOG_ENTRIES: MediumBlog =
        get_medium_blog(PathBuf::from(BLOG_ROOT), None, None);
    }

let this_blog = match all_blogs.hash.get(&complete_slug) {
    Some(x) => x,
    None => return None,
};

context.insert(
    "blog",
    &this_blog
        .render(PathBuf::from_str(BLOG_ROOT).unwrap())
        .unwrap(),
);
```
Finally, `low` stores absolutely nothing and is intended to be used to get
everything at runtime.

```rust,ignore
let preview = preview_blogs(PathBuf::from_str(BLOG_ROOT).unwrap(), 2, None);
let tags = get_blog_tag_list(PathBuf::from_str(BLOG_ROOT).unwrap());
let blog_post = render_blog_post(PathBuf::from_str(BLOG_ROOT).unwrap(), date, slug, None).unwrap();
```

This method can have serious runtime performance implecations, but might be
necessary if the blog can't fit into memory

## Examples

This crate comes with three examples - an identical blog website using
rocket and tera templates - one using each of the modules. You can run them
with

```bash,ignore
cargo run --example high
cargo run --example medium
cargo run --example low
```

You can then view them from localhost:8080