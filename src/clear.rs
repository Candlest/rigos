use std::fmt::format;

use crate::utils::*;

/* CLEAR
 * 在删除/public目录的同时，我们要保证.git文件不被破坏
*/
pub fn clear_all() {
    let git_DIR = format!("{}/.git", PUBLIC_DIR);
    let target_git_DIR = format!("{}/.git", STATIC_DIR);
    info(Info::CLEAR, "save to static", &target_git_DIR);
    copy_dir_all(git_DIR, target_git_DIR);
    let p: String = PUBLIC_DIR.to_string();
    info(Info::CLEAR, "clearing", PUBLIC_DIR);
    std::fs::remove_dir_all(std::path::Path::new(p.as_str()));
}

use std::{fs, io};

fn copy_dir_all(
    src: impl AsRef<std::path::Path>,
    dst: impl AsRef<std::path::Path>,
) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
