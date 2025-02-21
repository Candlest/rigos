use std::{fs::{self, File}, io::Write, path::Path};

use anyhow::Context;
use log::info;
use minijinja::{self, context, Environment};

use super::entities::{self, Post};

pub fn process_posts_and_index(env: &mut Environment, posts: &Vec<Post>) {
    std::fs::create_dir_all("public/post").unwrap();
    let temp = env.get_template("post.html").unwrap();
    for post in posts {
        let post_html = temp
            .render(context! {
                post => post,
            })
            .with_context(|| format!("Failed to render post: {}", post.filename)).unwrap();
        info!("Writing post: {}", post.filename);
        write_to_file(Path::new(&format!("public/post/{}.html", post.filename)), &post_html).unwrap();
    }
    let temp = env.get_template("index.html").unwrap();
    let index_html = temp
        .render(context! {
            posts => posts,
        })
        .unwrap();
    info!("Writing index.html");
    write_to_file(Path::new("public/index.html"), &index_html).unwrap();
}

pub fn process_pages(env: &mut Environment, pages: &Vec<entities::Page>) {
    info!("process pages...");
    let temp = env.get_template("page.html").unwrap();
    info!("pages: {:?}", pages);
    for page in pages {
        let page_html = temp
            .render(context! {
                page => page,
            })
            .unwrap();

        info!("Writing page: {}", page.filename);
        write_to_file(Path::new(&format!("public/{}.html", page.filename)), &page_html).unwrap();
    }
}

pub fn process_index(posts: &Vec<Post>) {
    info!("Writing posts.json");
    let posts_json = serde_json::to_string(&posts).unwrap();
    write_to_file(Path::new("public/posts.json"), &posts_json);
}

pub fn write_to_file(path: &Path, contents: &str) -> anyhow::Result<()> {
    let mut file = File::create(path).unwrap();
    file.write_all(contents.as_bytes())?;
    file.flush()?;
    Ok(())
}