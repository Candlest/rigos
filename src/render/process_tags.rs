use std::path::Path;

use anyhow::Context;
use log::info;
use minijinja::{self, context, Environment};

use super::entities::Post;
use std::collections::HashMap;

pub fn process_tags(env: &mut Environment, posts: &Vec<Post>) {
    info!("Processing tag pages...");
    let temp = env.get_template("tag.html").unwrap();

    let mut tag_map: HashMap<String, Vec<&Post>> = HashMap::new();
    for post in posts {
        for tag in &post.tags {
            tag_map.entry(tag.clone()).or_insert(Vec::new()).push(post);
        }
    }

    for (tag, tag_posts) in tag_map {
        let tag_html = temp
            .render(context! {
                tag => tag,
                posts => tag_posts,
            })
            .with_context(|| format!("Failed to render tag page: {}", tag))
            .unwrap();

        // 为了避免临时值在借用期间被丢弃的问题，先将格式化后的字符串绑定到一个变量上
        let tag_path_str = format!("public/tags/{}.html", tag);
        let tag_path = Path::new(&tag_path_str);
        info!("Writing tag page: {:?}", tag_path);
        std::fs::create_dir_all(tag_path.parent().unwrap()).unwrap();
        super::file_writer::write_to_file_utf8(tag_path, &tag_html).unwrap();
    }
}
