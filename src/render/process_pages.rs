use std::path::Path;

use log::info;
use minijinja::{self, context, Environment};

use super::entities::{Page, Post};
use super::process_posts::generate_common_data;

pub fn process_pages(env: &mut Environment, pages: &Vec<Page>, posts: &Vec<Post>) {
    let (recent_posts, tag_cloud, category_cloud) = generate_common_data(posts);
    info!("process pages...");
    let temp = env.get_template("page.html").unwrap();

    let temp_config =(*CONFIG).clone();  //解引用CONFIG使其可以被传递

    let html_template = env.get_template("page_pure.html").unwrap();
    info!("pages: {:?}", pages);
    for page in pages {
        let page_html = if page.is_html.unwrap_or(false) {
            html_template.render(context! { 
                page => page,
                recent_posts => recent_posts,
                tag_cloud => tag_cloud,
                category_cloud => category_cloud
           , CONFIG => temp_config, //以实现全局的客制化内容能被应用到各个页面 }).unwrap()
        } else {
            temp.render(context! { 
                page => page,
                recent_posts => recent_posts,
                tag_cloud => tag_cloud,
                category_cloud => category_cloud,
                CONFIG => temp_config,
            }).unwrap()
        };

        info!("Writing page: {}", page.filename);
        super::file_writer::write_to_file(
            Path::new(&format!("public/{}.html", page.filename)),
            &page_html,
        )
        .unwrap();
    }
}