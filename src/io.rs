use colored::{self, Colorize};
use std::fs::{self, create_dir_all, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn info(s: &str) {
    println!("{} {}", "[INFO]".bold(), s.bright_blue())
}

pub fn errstr(s: &str) -> String {
    format!("{} {}", "[ERRS]".bold(), s.bright_red())
}

pub fn read_file_contents(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_to_file(path: &Path, contents: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    file.flush()?;
    Ok(())
}

pub fn copy_dir_all(source: &Path, target: &Path) -> io::Result<()> {
    for entry in WalkDir::new(source) {
        let entry = entry?;
        let source_path = entry.path();
        let target_path = target.join(source_path.strip_prefix(source).unwrap());

        if source_path.is_file() {
            create_dir_all(target_path.parent().unwrap())?;
            fs::copy(source_path, &target_path)?;
        }
    }
    Ok(())
}
