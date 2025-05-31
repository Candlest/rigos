use std::path::Path;

use anyhow::Context;
use log::info;
use minijinja::{self, context, Environment};

use super::entities::Post;

// 删除常量 POSTS_PER_PAGE，因为不再需要分页

pub fn process_posts(env: &mut Environment, posts: &Vec<Post>) {
    std::fs::create_dir_all("public/post").unwrap();
    let temp = env.get_template("post.html").unwrap();

    // 直接克隆 posts 并排序
    let mut sorted_posts = posts.clone();
    sorted_posts.sort_by_key(|post| post.date);
    sorted_posts.reverse();
    let recent_posts = sorted_posts.iter().take(5).collect::<Vec<&Post>>();

    // 准备标签云数据
    let mut tag_cloud = std::collections::HashMap::new();
    for post in posts {
        for tag in &post.tags {
            *tag_cloud.entry(tag.clone()).or_insert(0) += 1;
        }
    }

    for post in posts {
        let post_html = temp
            .render(context! {
                post => post,
                recent_posts => recent_posts,
                tag_cloud => tag_cloud,
            })
            .with_context(|| format!("Failed to render post: {}", post.filename))
            .unwrap();
        info!("Writing post: {}", post.filename);
        super::file_writer::write_to_file(
            Path::new(&format!("public/post/{}.html", post.filename)),
            &post_html,
        )
        .unwrap();
    }
}

pub fn process_index(env: &mut Environment, posts: &Vec<Post>) {
    // 准备标签云和分类云数据
    let mut tag_cloud = std::collections::HashMap::new();
    let mut category_cloud = std::collections::HashMap::new();
    for post in posts {
        for tag in &post.tags {
            *tag_cloud.entry(tag.clone()).or_insert(0) += 1;
        }
        *category_cloud.entry(post.category.clone()).or_insert(0) += 1;
    }

    let ctx = minijinja::context! {
        posts => posts,
        tag_cloud => tag_cloud,
        category_cloud => category_cloud,
    };

    let rendered_index = env
        .get_template("index.html")
        .unwrap()
        .render(&ctx)
        .unwrap();
    let _ = super::file_writer::write_to_file_utf8(Path::new("public/index.html"), &rendered_index);
}
