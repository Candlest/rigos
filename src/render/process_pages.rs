use std::{fs::File, io::Write, path::Path};

use anyhow::Context;
use chrono::Datelike;
use log::info;
use minijinja::{self, context, Environment};

use super::entities::{self, Page};

pub fn process_pages(env: &mut Environment, pages: &Vec<Page>) {
    info!("process pages...");
    let temp = env.get_template("page.html").unwrap();
    let html_template = env.get_template("page_pure.html").unwrap();
    info!("pages: {:?}", pages);
    for page in pages {
        let page_html = if page.is_html.unwrap_or(false) {
            html_template.render(context! { page => page }).unwrap()
        } else {
            temp.render(context! { page => page }).unwrap()
        };

        info!("Writing page: {}", page.filename);
        super::file_writer::write_to_file(
            Path::new(&format!("public/{}.html", page.filename)),
            &page_html,
        )
        .unwrap();
    }
}