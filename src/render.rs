use crate::config;
use crate::io;
use chrono::NaiveDateTime;
use markdown::{self, CompileOptions, Options};
use minijinja::{context, Environment};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fs::create_dir_all;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{fmt, fs};
use toml;

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
    io::copy_dir_all(format!("theme/{}/js", theme), "pub/js".to_owned());
    io::copy_dir_all(format!("theme/{}/css", theme), "pub/css".to_owned());
    io::copy_dir_all(format!("theme/{}/static", theme), "pub".to_owned());
    io::copy_dir_all("assets".to_owned(), "pub/assets".to_owned());
}

fn process_posts(env: &mut Environment){
    let posts_dir = Path::new("posts");
    let mut list: Vec<Article> = Vec::new();
    let posts = read_posts_recursively(posts_dir); // read all posts recursively

    for post_path in posts {
        let content = fs::read_to_string(&post_path).unwrap();
        let parts: Vec<&str> = content.splitn(3, "---").collect();

        if parts.len() == 3 {
            let frontmatter = parts[1]; // frontmatter
            let markdown_content = parts[2];
            let html_content = markdown::to_html_with_options(
                &markdown_content,
                &Options {
                    compile: CompileOptions {
                        allow_dangerous_html: true,
                        allow_dangerous_protocol: true,
                        ..CompileOptions::gfm()
                    },
                    ..Options::gfm()
                },
            )
            .unwrap();

            let mut _article: Article = serde_yml::from_str(frontmatter).unwrap();
            _article.content = Some(html_content.clone());
            let html_name = _article.html_name.clone();

            let post_html = env
                .get_template("post")
                .unwrap()
                .render(context! {
                    article => _article.clone(),
                })
                .unwrap();
            // save to file and list
            list.push(_article);
            let _ = io::write_to_file(&format!("pub/post/{}.html", html_name), &post_html);
        } else {
            eprintln!(
                "File does not contain avaliable frontmatter: {:?}",
                post_path
            );
        }
    }

    list.sort_by(|a, b| b.date.cmp(&a.date));
    let post_json_content = env.get_template("json").unwrap()
        .render(context! {
            post_list => list,
        })
        .unwrap();
    // save to posts.json
    let _ = io::write_to_file("pub/posts.json", &post_json_content);
}

// fn generate_feed(env: &mut Environment, cfg: &config::Config, posts: &[Article]) {
//     if cfg.rss_page == Some(true) {
//         let template_feed = read_template_file(&format!("theme/{}/feed.xml", cfg.theme)).unwrap();
//         env.add_template("feed", template_feed).unwrap();
//         let template_feed = env.get_template("feed").unwrap();
//         let feed_xml = template_feed
//             .render(context! {
//                 site_title => cfg.site_title,
//                 site_link => cfg.site_link,
//                 site_description => cfg.site_description,
//                 posts => posts.to_vec()
//             })
//             .unwrap();
//         let _ = io::write_to_file("pub/feed.xml", &feed_xml);
//     }
// }

fn render_pages(env: &mut Environment, cfg: &config::Config, theme: &str) {
    for page in &cfg.pages {
        let content = io::read_file_contents(&format!("{}.md", page))
            .unwrap()
            .to_string();
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() == 3 {
            let frontmatter = parts[1];
            let markdown_content = parts[2];
            let html_content = markdown::to_html_with_options(
                &markdown_content,
                &Options {
                    compile: CompileOptions {
                        allow_dangerous_html: true,
                        allow_dangerous_protocol: true,
                        ..CompileOptions::gfm()
                    },
                    ..Options::gfm()
                },
            )
            .unwrap();

            let mut page: Page = serde_yml::from_str(&frontmatter).unwrap();
            page.content = Some(html_content.clone());
            let page_html = env.get_template("page").unwrap()
                .render(context! {
                    page => page.clone(),
                })
                .unwrap();
            let _ = io::write_to_file(&format!("pub/{}.html", page.html_name), &page_html);
        } else {
            eprintln!("File does not contain right '---' separator: {:?}", page);
        }
    }
}

fn read_template_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
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