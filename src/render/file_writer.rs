use std::{fs::File, io::Write, path::Path};
use anyhow::Result;

pub fn write_to_file_utf8(path: &Path, contents: &str) -> Result<()> {
    let mut file = File::create(path).unwrap();
    file.write_all(contents.as_bytes())?;
    file.flush()?;
    Ok(())
}

pub fn write_to_file(path: &Path, contents: &str) -> Result<()> {
    let mut file = File::create(path).unwrap();
    file.write_all(contents.as_bytes())?;
    file.flush()?;
    Ok(())
}