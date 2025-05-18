use std::{
    fs::{self},
    io::Read,
    path::Path,
};

use crate::{ASSETS_PATH, CONFIG, PAGE_PATH, POST_PATH};
use anyhow::Context;
use entities::{Page, Post};
use log::info;
use minijinja::{self, Environment};
use serde_yml;
use walkdir::WalkDir;
mod entities;
mod process;
use markdown::{to_html_with_options, CompileOptions, Options};

pub fn render_all() {
    let pages = build_pages();
    let posts = build_posts();

    copy_dir_all(&*ASSETS_PATH, Path::new("public/assets"))
        .with_context(|| {
            format!(
                "Failed to copy assets from {:?} to {:?}",
                ASSETS_PATH, "public/assets"
            )
        })
        .unwrap();

    // theme
    let static_dir = format!("theme/{}/static", CONFIG.theme);
    let dest_dir = "public";
    copy_dir_all(Path::new(&static_dir), Path::new(&dest_dir))
        .with_context(|| {
            format!(
                "Failed to copy theme files from {:?} to {:?}",
                static_dir, dest_dir
            )
        })
        .unwrap();

    let templates_dir = format!("theme/{}/templates", CONFIG.theme);
    let mut env = Environment::new();
    let template_base = read_template_file(&format!("{}/base.html", templates_dir)).unwrap();
    env.add_template("base.html", template_base.as_str())
        .unwrap();
    let template_page = read_template_file(&format!("{}/page.html", templates_dir)).unwrap();
    env.add_template("page.html", template_page.as_str())
        .unwrap();
    let template_post = read_template_file(&format!("{}/post.html", templates_dir)).unwrap();
    env.add_template("post.html", template_post.as_str())
        .unwrap();
    let template_index = read_template_file(&format!("{}/index.html", templates_dir)).unwrap();
    env.add_template("index.html", template_index.as_str())
        .unwrap();
    let template_index = read_template_file(&format!("{}/archive.html", templates_dir)).unwrap();
    env.add_template("archive.html", template_index.as_str())
        .unwrap();
    process::process_posts_and_index(&mut env, &posts);
    process::process_archive(&mut env, &posts);
    process::process_pages(&mut env, &pages);
}

pub fn build_pages() -> Vec<Page> {
    let mut pages = vec![];
    info!("Building pages...");
    for entry in WalkDir::new(&*PAGE_PATH).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "md" {
                    if path.file_name().unwrap() == "index.md" {
                        continue;
                    }
                    let source = std::fs::read_to_string(path)
                        .with_context(|| format!("Failed to read file: {:?}", path))
                        .unwrap();
                    let (frontmatter, markdown_content) = split_content(&source);
                    let mut page: Page = serde_yml::from_str(&frontmatter)
                        .with_context(|| frontmatter.to_string())
                        .unwrap();
                    page.content = Some(markdown_to_html(markdown_content));
                    pages.push(page);
                } else {
                    let dest_path = "public/";
                    std::fs::create_dir_all(
                        std::path::Path::new(&dest_path)
                            .parent()
                            .with_context(|| {
                                format!("Failed to get parent of path: {:?}", dest_path)
                            })
                            .unwrap(),
                    )
                    .with_context(|| format!("Failed to create directory: {:?}", dest_path))
                    .unwrap();
                    std::fs::copy(path, dest_path)
                        .with_context(|| format!("Failed to copy file: {:?}", path))
                        .unwrap();
                }
            }
        }
    }
    pages // TODO: sort by datetime
}

pub fn build_posts() -> Vec<Post> {
    let mut posts = vec![];
    info!("Building posts...");
    for entry in WalkDir::new(&*POST_PATH).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "md" {
                    let source = std::fs::read_to_string(path)
                        .with_context(|| format!("Failed to read file: {:?}", path))
                        .unwrap();
                    let (frontmatter, markdown_content) = split_content(&source);
                    let mut post: Post = serde_yml::from_str(&frontmatter)
                        .with_context(|| frontmatter.to_string())
                        .unwrap();
                    post.content = Some(markdown_to_html(markdown_content));
                    posts.push(post);
                } else {
                    let dest_path = format!(
                        "public/{}",
                        path.strip_prefix(&*POST_PATH)
                            .with_context(|| format!(
                                "Failed to strip prefix from path: {:?}",
                                path
                            ))
                            .unwrap()
                            .to_str()
                            .unwrap()
                    );
                    std::fs::create_dir_all(
                        std::path::Path::new(&dest_path)
                            .parent()
                            .with_context(|| {
                                format!("Failed to get parent of path: {:?}", dest_path)
                            })
                            .unwrap(),
                    )
                    .with_context(|| format!("Failed to create directory: {:?}", dest_path))
                    .unwrap();
                    std::fs::copy(path, dest_path)
                        .with_context(|| format!("Failed to copy file: {:?}", path))
                        .unwrap();
                }
            }
        }
    }
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    posts
}

fn split_content(s: &str) -> (&str, &str) {
    // frontmatter 是第一个 --- 和第二个 --- 之间的内容
    // markdown_content 是第二个 --- 之后的内容
    let mut split = s.splitn(3, "---");
    let frontmatter = split.next().unwrap_or("");
    let frontmatter = split.next().unwrap_or("");
    let markdown_content = split.next().unwrap_or("");
    (frontmatter, markdown_content)
}


fn markdown_to_html(markdown_input: &str) -> String {
    to_html_with_options(markdown_input, &Options {
        compile: CompileOptions {
          allow_dangerous_html: true,
          allow_dangerous_protocol: true,
            ..CompileOptions::gfm()
        },
        ..Options::default()
    }).unwrap()
}

fn copy_dir_all(src: &Path, dst: &Path) -> anyhow::Result<()> {
    for entry in WalkDir::new(src) {
        let entry = entry?;
        let src_path = entry.path();
        let relative_path = src_path.strip_prefix(src)?;
        let dst_path = dst.join(relative_path);

        if entry.file_type().is_dir() {
            // 如果是文件夹，创建目标文件夹
            if !dst_path.exists() {
                fs::create_dir_all(&dst_path)?;
            }
        } else {
            // 如果是文件，复制文件
            fs::copy(src_path, dst_path)?;
        }
    }
    Ok(())
}

fn read_template_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    info!("Reading template file: {}", path);
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
