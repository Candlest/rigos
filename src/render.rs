use crate::config;
use crate::io;
use chrono::NaiveDateTime;
use markdown::{self, CompileOptions, Options};
use minijinja::{context, Environment};
use pulldown_cmark::html;
use pulldown_cmark::TagEnd;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::{fmt, fs};

#[derive(Deserialize, Serialize, Clone)]
pub struct Article {
    title: String,
    html_name: String,
    date: NaiveDateTime,
    latest: Option<NaiveDateTime>,
    published: Option<bool>,
    tags: Vec<String>,
    category: String,
    content: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Page {
    title: String,
    html_name: String,
    content: Option<String>,
}

pub async fn render() {
    let cfg = config::read_config("config.toml").unwrap();
    let theme = cfg.theme.clone();

    create_directories(); // create directories
    copy_static_files(&theme); // copy static files
    let mut env = Environment::new();
    let template_base = read_template_file(&format!("theme/{}/base.html", theme)).unwrap();
    env.add_template("base", template_base.as_str()).unwrap();
    let template_page = read_template_file(&format!("theme/{}/page.html", theme)).unwrap();
    env.add_template("page", template_page.as_str()).unwrap();
    let template_post = read_template_file(&format!("theme/{}/post.html", theme)).unwrap();
    env.add_template("post", template_post.as_str()).unwrap();
    let template_json = read_template_file(&format!("theme/{}/posts.json", theme)).unwrap();
    env.add_template("json", template_json.as_str()).unwrap();

    // deal with posts
    process_posts(&mut env);
    // generate_feed(&mut env, &cfg, &posts);
    render_pages(&mut env, &cfg, &theme);
}

fn create_directories() {
    let _ = create_dir_all("pub");
    let _ = create_dir_all("pub/post");
}

fn copy_static_files(theme: &str) {
    io::copy_dir_all(Path::new(&format!("theme/{}/js", theme)), Path::new("pub/js"));
    io::copy_dir_all(Path::new(&format!("theme/{}/css", theme)), Path::new("pub/css"));
    io::copy_dir_all(Path::new(&format!("theme/{}/static", theme)), Path::new("pub"));
    io::copy_dir_all(Path::new("assets"), Path::new("pub/assets"));
}

fn read_template_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn process_posts(env: &mut Environment) {
    let posts_dir = Path::new("posts");
    let mut list: Vec<Article> = Vec::new();
    let posts = read_posts_recursively(posts_dir); // read all posts recursively

    for post_path in posts {
        if let Ok(content) = fs::read_to_string(&post_path) {
            if let Some((frontmatter, markdown_content)) = split_content(&content) {
                let html_content = markdown_to_html(markdown_content);
                let mut article: Article = serde_yml::from_str(frontmatter).unwrap();
                article.content = Some(html_content.clone());
                let html_name = article.html_name.clone();

                let post_html = env
                    .get_template("post")
                    .unwrap()
                    .render(context! {
                        article => article.clone(),
                    })
                    .unwrap();

                if article.published != Some(false) {
                    list.push(article);
                }
                let _ = io::write_to_file(Path::new(&format!("pub/post/{}.html", html_name)), &post_html);
            } else {
                eprintln!("File does not contain valid frontmatter: {:?}", post_path);
            }
        } else {
            eprintln!("Failed to read file: {:?}", post_path);
        }
    }

    list.sort_by(|a, b| b.date.cmp(&a.date));
    let post_json_content = env
        .get_template("json")
        .unwrap()
        .render(context! {
            post_list => list,
        })
        .unwrap();
    let _ = io::write_to_file(Path::new("pub/posts.json"), &post_json_content);
}

fn render_pages(env: &mut Environment, cfg: &config::Config, theme: &str) {
    for page in &cfg.pages {
        if let Ok(content) = io::read_file_contents(Path::new(&format!("{}.md", page))) {
            if let Some((frontmatter, markdown_content)) = split_content(&content) {
                let html_content = markdown_to_html(markdown_content);
                let mut page: Page = serde_yml::from_str(&frontmatter).unwrap();
                page.content = Some(html_content.clone());

                let page_html = env
                    .get_template("page")
                    .unwrap()
                    .render(context! {
                        page => page.clone(),
                    })
                    .unwrap();

                let _ = io::write_to_file(Path::new(&format!("pub/{}.html", page.html_name)), &page_html);
            } else {
                eprintln!("File does not contain valid frontmatter: {:?}", page);
            }
        } else {
            eprintln!("Failed to read file: {:?}", page);
        }
    }
}

fn read_posts_recursively(dir: &Path) -> Vec<PathBuf> {
    let mut posts = Vec::new();
    if dir.is_dir() {
        let entries = fs::read_dir(dir).unwrap();
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                posts.append(&mut read_posts_recursively(&path));
            } else if path.is_file() && path.extension().unwrap() == "md" {
                posts.push(path);
            }
        }
    }
    posts
}

fn split_content(content: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() == 3 {
        Some((parts[1], parts[2]))
    } else {
        None
    }
}

use pulldown_cmark::{Event, Options as CmarkOptions, Parser, Tag};

fn markdown_to_html(markdown_input: &str) -> String {
    // 启用 GFM 选项
    let options = CmarkOptions::all();
    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}