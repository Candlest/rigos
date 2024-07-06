use colored::{self, Colorize};
use std::fs::{self, create_dir_all, File};
use std::io::{self, Read, Write};
use std::path::{Path};
use walkdir::WalkDir;

pub fn info(s: &str) {
    println!("{} {}", "[INFO]".bold(), s.bright_blue())
}

pub fn errstr(s: &str) -> String {
    format!("{} {}", "[ERRS]".bold(), s.bright_red())
}

pub fn read_file_contents(path: &str) -> io::Result<String> {
    // 尝试打开文件
    let mut file = File::open(path)?;

    // 创建一个字符串变量来存储文件内容
    let mut contents = String::new();

    // 读取文件到字符串
    file.read_to_string(&mut contents)?;

    // 返回文件内容
    Ok(contents)
}

pub fn write_to_file(path: &str, contents: &str) -> io::Result<()> {
    // 创建或获取一个可写文件句柄
    let mut file = File::create(path)?;

    // 将字符串写入文件
    file.write_all(contents.as_bytes())?;

    // 刷新文件以确保所有数据都被写入
    file.flush()?;

    Ok(())
}

pub fn copy_dir_all(p1: String, p2: String) {
    let source_dir = Path::new(&p1);
    let target_dir = Path::new(&p2);

    for entry in WalkDir::new(source_dir) {
        let entry = entry.unwrap();
        let source_path = entry.path();
        let target_path = target_dir.join(source_path.strip_prefix(source_dir).unwrap());

        if source_path.is_file() {
            let _ = create_dir_all(target_path.parent().as_ref().unwrap());
            fs::copy(source_path, target_path.clone()).expect(&errstr(&format!(
                "Failed to copy file!{:?}to{:?}",
                source_path, target_path
            )));
        }
    }
}