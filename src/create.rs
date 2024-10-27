use std::{fs::create_dir_all, path::Path, path::PathBuf, process::Command};

use chrono::Local;

use crate::{
    config,
    io::{self, info},
};

pub fn create_new_page(name: String) {
    let _ = io::write_to_file(Path::new(&format!("./{}.md", name)), include_str!("templates/page.md"));
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
    let raw_content = include_str!("templates/post.md");
    let contents = raw_content
        .replace("{file_name}", file_name)
        .replace("{formatted}", &formatted)
        .replace("{parent_path}", parent_path);
    let _ = create_dir_all(&format!("./posts/{}", parent_path));
    let _ = io::write_to_file(Path::new(&format!("./posts/{}.md", opath)), &contents);
    io::info("new post created");
}

pub fn create_new_site() {
    let work_dir = PathBuf::from(".");
    // 创建命令
    let mut child = Command::new("git")
        .current_dir(&work_dir)
        .arg("clone") // 添加参数
        .arg("https://github.com/Candlest/rigos-template.git")
        .arg(".")
        .spawn() // 启动子进程
        .expect("failed to clone template");

    // 等待命令执行完成
    child.wait().expect("failed to clone template");

    let work_dir = PathBuf::from("pub");
    // 创建命令
    let mut child = Command::new("git")
        .current_dir(&work_dir)
        .arg("init") // 添加参数
        .spawn() // 启动子进程
        .expect("failed to init repo");

    let status = child.wait().expect("failed to init repo");

    // 打印命令的退出状态
    io::info(format!("Command finished with status: {}", status).as_str());
}
