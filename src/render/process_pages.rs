use std::path::Path;

use log::info;
use minijinja::{self, context, Environment};

use crate::{CONFIG};

use super::entities::Page;

pub fn process_pages(env: &mut Environment, pages: &Vec<Page>) {
    info!("process pages...");
    let temp = env.get_template("page.html").unwrap();

    let temp_config =(*CONFIG).clone();  //解引用CONFIG使其可以被传递

    let html_template = env.get_template("page_pure.html").unwrap();
    info!("pages: {:?}", pages);
    for page in pages {
        let page_html = if page.is_html.unwrap_or(false) {
            html_template.render(context! { page => page, CONFIG => temp_config, //以实现全局的客制化内容能被应用到各个页面 }).unwrap()
        } else {
            temp.render(context! { page => page, CONFIG => temp_config, //以实现全局的客制化内容能被应用到各个页面 }).unwrap()
        };

        info!("Writing page: {}", page.filename);
        super::file_writer::write_to_file(
            Path::new(&format!("public/{}.html", page.filename)),
            &page_html,
        )
        .unwrap();
    }
}