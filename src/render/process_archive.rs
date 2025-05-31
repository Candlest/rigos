use std::path::Path;

use anyhow::Context;
use chrono::Datelike;
use log::info;
use minijinja::{self, context, Environment};

use super::entities::Post;
use std::collections::BTreeMap;

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

    // println!("Debug archive_data: {:?}", archive_data);

    let archive_html = temp
        .render(context! {
            archive_data => archive_data,
        })
        .with_context(|| "Failed to render archive page".to_string())
        .unwrap();

    // println!("Debug archive_html: {}", archive_html);

    let archive_path = Path::new("public/archive.html");
    info!("Writing archive page: {:?}", archive_path);
    super::file_writer::write_to_file_utf8(archive_path, &archive_html).unwrap();
}