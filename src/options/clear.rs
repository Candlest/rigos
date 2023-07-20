use crate::utils::*;

pub fn clear_all() {
    let p: String = PUBLIC_DIR.to_string();
    info(Info::CLEAR, "clearing", PUBLIC_DIR);
    std::fs::remove_dir_all(std::path::Path::new(p.as_str()));
}
