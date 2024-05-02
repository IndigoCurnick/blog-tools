use std::borrow::Borrow;

pub fn get_preview(html: &String, preview_chars: Option<usize>) -> String {
    let num_chars = match preview_chars {
        Some(x) => x,
        None => 320,
    };

    let dom = tl::parse(&html, tl::ParserOptions::default()).unwrap();

    let parser = dom.parser();

    let mut preview = String::new();

    for node in dom.nodes() {
        println!("Now deadling with {:?}", node);
        let tag = match node.as_tag() {
            Some(x) => x,
            None => continue,
        };

        if tag.name() == "p" {
            preview.push_str(node.inner_text(parser).borrow());

            if preview.len() > num_chars {
                break;
            }
        }
    }

    if preview.len() > num_chars {
        preview.truncate(num_chars);
    }

    return preview;
}
