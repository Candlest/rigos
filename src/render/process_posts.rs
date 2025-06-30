use std::path::Path;

use anyhow::Context;
use log::info;
use minijinja::{self, context, Environment};

use crate::{CONFIG};

use super::entities::Post;
use std::collections::HashMap;

pub fn process_posts(env: &mut Environment, posts: &Vec<Post>){
    std::fs::create_dir_all("public/post").unwrap();
    let temp = env.get_template("post.html").unwrap();

    let (recent_posts, tag_cloud, category_cloud) = generate_common_data(posts);

    let temp_config =(*CONFIG).clone();  //解引用CONFIG使其可以被传递

    for post in posts {
        let post_html = temp
            .render(context! {
                post => post,
                recent_posts => recent_posts,
                tag_cloud => tag_cloud,
                category_cloud => category_cloud,
                CONFIG => temp_config,
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

    // 对文章按日期排序（最新的在前）
    let mut sorted_posts = posts.clone();
    sorted_posts.sort_by_key(|post| post.date.clone());
    sorted_posts.reverse();

    let POSTS_PER_PAGE: usize = super::CONFIG.posts_number_in_each_page; // 每页显示5篇文章
    let total_posts = sorted_posts.len();
    let total_pages = (total_posts + POSTS_PER_PAGE - 1) / POSTS_PER_PAGE;

    // 创建分页目录
    std::fs::create_dir_all("public/index").unwrap();

    for page in 1..=total_pages {
        let start = (page - 1) * POSTS_PER_PAGE;
        let end = start + POSTS_PER_PAGE;
        let paginated_posts = &sorted_posts[start..end.min(total_posts)];

        let ctx = minijinja::context! {
            posts => paginated_posts,
            tag_cloud => &tag_cloud,
            category_cloud => &category_cloud,
            current_page => page,
            total_pages => total_pages,
            has_previous => page > 1,
            has_next => page < total_pages,
        };

        let rendered_index = env
            .get_template("index.html")
            .unwrap()
            .render(&ctx)
            .unwrap();

        let path_str = format!("public/index/{}.html", page);
        let output_path = if page == 1 {
            Path::new("public/index.html")
        } else {
            Path::new(path_str.as_str())
        };

        super::file_writer::write_to_file_utf8(output_path, &rendered_index).unwrap();
    }
}

/// Generates recent_posts, tag_cloud, and category_cloud
pub fn generate_common_data(posts: &Vec<Post>) -> (Vec<Post>, HashMap<String, i32>, HashMap<String, i32>) {
    let mut sorted_posts = posts.clone();
    sorted_posts.sort_by_key(|post| post.date.clone()); // `post.date` is also being borrowed, so clone it
    sorted_posts.reverse();
    let recent_posts = sorted_posts.iter().take(5).cloned().collect::<Vec<Post>>();

    let mut tag_cloud = HashMap::new();
    let mut category_cloud = HashMap::new();
    for post in posts {
        for tag in &post.tags {
            *tag_cloud.entry(tag.clone()).or_insert(0) += 1;
        }
        *category_cloud.entry(post.category.clone()).or_insert(0) += 1;
    }

    return (recent_posts, tag_cloud, category_cloud);
}