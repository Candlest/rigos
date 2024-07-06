use std::{fs::create_dir_all, path::Path};

use chrono::Local;

use crate::{config, io};

pub fn create_new_page(name: String) {
    let _ = io::write_to_file(&format!("./{}.md", name), "# New Page");
    // 注册
    let mut cfg = config::read_config("config.toml").unwrap();
    cfg.pages.push(name);
    config::write_config("config.toml", &cfg).unwrap();
    io::info("new page created");
}

pub fn create_new_post(opath: String) {

    let now = Local::now();
    let formatted = now.format("%Y-%m-%dT%H:%M:%S").to_string();

    let path = Path::new(&opath);

    // 获取文件名
    let file_name = path
        .file_name()
        .expect("Path must have a file name")
        .to_str()
        .expect("File name must be valid UTF-8");

    // 获取路径部分
    let parent_path = path
        .parent()
        .expect("Path must have a parent")
        .to_str()
        .expect("Parent path must be valid UTF-8");

    println!("File name: {}", file_name);
    println!("Parent path: {}", parent_path);
    let contents = format!(
        "
title=\"{}\"
filename=\"{}\"
date=\"{}\"
tags=[]
category=\"{}\"
%%%%%%
",
        file_name, file_name, formatted ,parent_path
    );
    let _ = create_dir_all(&format!("./posts/{}", parent_path));
    let _ = io::write_to_file(&format!("./posts/{}.md", opath), &contents);
    io::info("new post created");
}
