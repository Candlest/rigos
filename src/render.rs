use crate::config;
use crate::io;
use chrono::{NaiveDateTime};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use markdown::{self, CompileOptions, Options};
use minijinja::{context, Environment};
use toml;
use std::collections::{HashMap};
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::{fmt, fs};
use std::io::Read;

#[derive(Deserialize, Serialize, Clone)]
pub struct Page {
    info: PageInfo,
    contents: String,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct PageInfo {
    title: String,
    filename: String,
}

#[derive(Serialize, Clone)]
pub struct Post {
    info: PostInfo,
    contents: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PostInfo {
    title: String,
    filename: String,
    date: NaiveDateTime,
    tags: Vec<String>,
    category: String,
}

pub fn render() {
    let cfg = config::read_config("config.toml").unwrap();
    let theme = cfg.theme;

    // 复制静态文件
    io::copy_dir_all(format!("theme/{}/static", theme), "pub".to_owned());

    // 注册
    let mut env = Environment::new();

    // 添加base.html
    let template_base = read_template_file(format!("theme/{}/base.html", theme).as_str()).unwrap();
    env.add_template("base", &template_base).unwrap();

    // 渲染文章

    // 确保文件夹存在
    let _ = create_dir_all("pub/post");

    let template_post = read_template_file(format!("theme/{}/post.html", theme).as_str()).unwrap();
    env.add_template("post", &template_post).unwrap();
    let template_post = env.get_template("post").unwrap();
    // let mut list: Vec<Post> = Vec::new();
    let list: Vec<Post> = Vec::new();
    let posts_dir = Path::new("posts");
    let mut list: Vec<Post> = Vec::new();
    let posts = read_posts_recursively(posts_dir);
    for post_path in posts {
        if post_path.is_file() {
            // 读取文件内容
            let content = fs::read_to_string(&post_path).unwrap();
    
            // 按 ++++++ 分割内容
            let parts: Vec<&str> = content.split("++++++").collect();
    
            if parts.len() == 3 {
                let toml_content = parts[1].trim();
                let markdown_content = parts[2].trim();
                let markdown_content_html = markdown::to_html_with_options(
                    &markdown_content,
                    &Options {
                        compile: CompileOptions {
                            allow_dangerous_html: true,
                            allow_dangerous_protocol: true,
                            ..CompileOptions::default()
                        },
                        ..Options::default()
                    },
                )
                .unwrap();
    
                // 从 TOML 元数据解析 PostInfo
                let info: PostInfo = read_toml_to_config(toml_content).unwrap();
    
                // 创建 Post 对象
                let post_obj = Post {
                    info: info.clone(),
                    contents: markdown_content_html.clone(),
                };
                list.push(post_obj);
    
                // 使用模板渲染 HTML
                let post_html = template_post
                    .render(context! {
                        contents => markdown_content_html.clone(),
                        info => info.clone(),
                    })
                    .unwrap();
    
                // 写入 HTML 文件
                let filename = info.filename.clone();
                let _ = io::write_to_file(&format!("pub/post/{}.html", filename), &post_html);
            } else {
                eprintln!("File does not contain right '++++++' separator: {:?}", post_path);
            }
        }
    }

    // list 按时间排序
    list.sort_by(|a, b| b.info.date.cmp(&a.info.date));

    // feed.xml
    let template_feed = read_template_file(format!("theme/{}/feed.xml", theme).as_str()).unwrap();
    env.add_template("feed", &template_feed).unwrap();
    let template_feed = env.get_template("feed").unwrap();
    let feed_xml = template_feed.render(context! {
        site_title => cfg.site_title,
        site_link => cfg.site_link,
        site_description => cfg.site_description,
        posts => list.clone()
    }).unwrap();
    let _ = io::write_to_file("pub/feed.xml", &feed_xml);

    // 分类文章
    // 创建一个 HashMap 来按标签分类文章
    let mut tags_dict: HashMap<String, Vec<Post>> = HashMap::new();
    // 遍历文章列表并按标签分类
    for post in list.iter() {
        for tag in &post.info.tags {
            tags_dict
                .entry(tag.clone())
                .or_insert_with(Vec::new)
                .push(post.clone());
        }
    }

    // 创建一个 HashMap 来按分类分组文章
    let mut categories_dict: HashMap<String, Vec<Post>> = HashMap::new();

    // 遍历文章列表并按分类分组
    for post in list.clone() {
        categories_dict
            .entry(post.info.category.clone())
            .or_insert_with(Vec::new)
            .push(post.clone());
    }

    // 渲染 page
    let template_page = read_template_file(format!("theme/{}/page.html", theme).as_str()).unwrap();
    env.add_template("page", &template_page).unwrap();
    let template_page = env.get_template("page").unwrap();
    for page in cfg.pages {
        let content = io::read_file_contents(&format!("{}.md", page))
            .unwrap()
            .to_string();
        let parts: Vec<&str> = content.split("%%%%%%").collect();
        if parts.len() == 2 {
            let toml_content = parts[0].trim().to_string();
            let markdown_content = parts[1].trim().to_string();
            let markdown_content = markdown::to_html_with_options(
                &markdown_content.as_str(),
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

            // 保存
            let info = toml::from_str::<PageInfo>(&toml_content).unwrap();
            // 输出
            let page_html = template_page
                .render(context! {
                    contents => markdown_content,
                    info => info,
                })
                .unwrap();
            let _ = io::write_to_file(&format!("pub/{}.html", info.filename), &page_html);
        } else {
            eprintln!("File does not contain '%%%%%%' separator: {:?}", page);
        }
    }

    // TO DO: 渲染 feed.xml

    // 渲染 index.html
    let template_index =
        read_template_file(format!("theme/{}/index.html", theme).as_str()).unwrap();
    env.add_template("page", &template_index).unwrap();
    let template_index = env.get_template("page").unwrap();
    let content = io::read_file_contents("index.md").unwrap().to_string();
    let content = markdown::to_html_with_options(
        &content.as_str(),
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
    // let index_data = serde_json::json!(
    //     {
    //         "contents" : content,
    //         "list" : list
    //     }
    // );
    //println!("{}", serde_json::to_string_pretty(&index_data).unwrap().clone());
    let index_html = template_index
        .render(context! {
            contents => content,
            list => list,
            categories_dict => categories_dict,
            tags_dict => tags_dict
        })
        .unwrap();
    let _ = io::write_to_file("pub/index.html", &index_html);
}

fn read_template_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// 自定义 Deserializer 来解析 TOML 中的日期时间字符串

struct NaiveDateTimeVisitor;

impl<'de> Visitor<'de> for NaiveDateTimeVisitor {
    type Value = NaiveDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a date and time in ISO 8601 format")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        // 尝试解析字符串为 NaiveDateTime
        NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S")
            .map_err(E::custom)
    }
}

fn deserialize_naive_date_time<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(NaiveDateTimeVisitor)
}

// 然后，在反序列化过程中使用 deserialize_naive_date_time 函数
fn read_toml_to_config(toml_str: &str) -> Result<PostInfo, toml::de::Error> {
    let article: PostInfo = toml::from_str(toml_str)?;
    Ok(article)
}

fn read_posts_recursively(dir: &Path) -> Vec<PathBuf> {
    let mut posts = Vec::new();
    if dir.is_dir() {
        let entries = fs::read_dir(dir).unwrap();
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // 如果是目录，递归调用
                posts.append(&mut read_posts_recursively(&path));
            } else if path.is_file() {
                // 如果是文件，添加到列表
                posts.push(path);
            }
        }
    }
    posts
}
