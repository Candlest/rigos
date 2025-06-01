use std::path::Path;

use anyhow::Context;
use log::info;
use minijinja::{self, context, Environment};

use crate::{ CONFIG };

use super::entities::Post;
use std::collections::HashMap;

pub fn process_categories(env: &mut Environment, posts: &Vec<Post>) {
    info!("Processing category pages...");
    let temp = env.get_template("category.html").unwrap();

    let mut category_map: HashMap<String, Vec<&Post>> = HashMap::new();
    for post in posts {
        category_map.entry(post.category.clone()).or_insert(Vec::new()).push(post);
    }

    let temp_config =(*CONFIG).clone();  //解引用CONFIG使其可以被传递
    
    for (category, category_posts) in category_map {
        let category_html = temp
            .render(context! {
                category => category,
                posts => category_posts,
                CONFIG => temp_config, //以实现全局的客制化内容能被应用到各个页面
            })
            .with_context(|| format!("Failed to render category page: {}", category.clone()))
            .unwrap();
        let category_path_str = format!("public/categories/{}.html", category);
        let category_path = Path::new(&category_path_str);
        info!("Writing category page: {:?}", category_path);
        std::fs::create_dir_all(category_path.parent().unwrap()).unwrap();
        super::file_writer::write_to_file_utf8(category_path, &category_html).unwrap();
    }
}