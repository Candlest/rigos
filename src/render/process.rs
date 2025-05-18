use std::{fs::File, io::Write, path::Path};

use anyhow::Context;
use chrono::Datelike;
use log::info;
use minijinja::{self, context, Environment};

use super::entities::{self, Post};
use std::collections::BTreeMap;

const POSTS_PER_PAGE: usize = 5;

pub fn process_posts_and_index(env: &mut Environment, posts: &Vec<Post>) {
    std::fs::create_dir_all("public/post").unwrap();
    let temp = env.get_template("post.html").unwrap();
    for post in posts {
        let post_html = temp
            .render(context! {
                post => post,
            })
            .with_context(|| format!("Failed to render post: {}", post.filename))
            .unwrap();
        info!("Writing post: {}", post.filename);
        write_to_file(
            Path::new(&format!("public/post/{}.html", post.filename)),
            &post_html,
        )
        .unwrap();
    }

    let total_pages = (posts.len() + POSTS_PER_PAGE - 1) / POSTS_PER_PAGE;
    for page_num in 1..=total_pages {
        let start_index = (page_num - 1) * POSTS_PER_PAGE;
        let end_index = start_index + POSTS_PER_PAGE;
        let paginated_posts = &posts[start_index..end_index.min(posts.len())];

        let temp = env.get_template("index.html").unwrap();
        let index_html = temp
            .render(context! {
                posts => paginated_posts,
                page => page_num,
                total_pages => total_pages
            })
            .unwrap();

        let index_filename = if page_num == 1 {
            "public/index.html".to_string()
        } else {
            format!("public/index_{}.html", page_num)
        };
        info!("Writing index page: {}", index_filename);
        write_to_file(Path::new(&index_filename), &index_html).unwrap();
    }
}

pub fn process_archive(env: &mut Environment, posts: &Vec<Post>) {
    info!("Processing archive page...");
    let temp = env.get_template("archive.html").unwrap();

    let mut archive_map: BTreeMap<i32, BTreeMap<u32, Vec<&Post>>> = BTreeMap::new();
    for post in posts {
        let year = post.date.year();
        let month = post.date.month();
        archive_map
            .entry(year)
            .or_default()
            .entry(month)
            .or_default()
            .push(post);
    }

    let mut archive_data: Vec<(i32, Vec<(u32, Vec<&Post>)>)> = archive_map
        .into_iter()
        .map(|(year, months)| {
            let mut months_vec: Vec<(u32, Vec<&Post>)> = months.into_iter().collect();
            months_vec.sort_by(|a, b| b.0.cmp(&a.0)); // Sort months in descending order
            (year, months_vec)
        })
        .collect();

    archive_data.sort_by(|a, b| b.0.cmp(&a.0)); // Sort years in descending order

    println!("Debug archive_data: {:?}", archive_data);

    let archive_html = temp
        .render(context! {
            archive_data => archive_data,
        })
        .with_context(|| "Failed to render archive page".to_string())
        .unwrap();

    println!("Debug archive_html: {}", archive_html);

    let archive_path = Path::new("public/archive.html");
    info!("Writing archive page: {:?}", archive_path);
    write_to_file_utf8(archive_path, &archive_html).unwrap();
}

pub fn write_to_file_utf8(path: &Path, contents: &str) -> anyhow::Result<()> {
    let mut file = File::create(path).unwrap();
    file.write_all(contents.as_bytes())?;
    file.flush()?;
    Ok(())
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
        write_to_file(
            Path::new(&format!("public/{}.html", page.filename)),
            &page_html,
        )
        .unwrap();
    }
}

pub fn write_to_file(path: &Path, contents: &str) -> anyhow::Result<()> {
    let mut file = File::create(path).unwrap();
    file.write_all(contents.as_bytes())?;
    file.flush()?;
    Ok(())
}
