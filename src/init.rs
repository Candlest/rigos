/* 生成新的project
 * 
 */

use std::path;
use std::fs;
use crate::utils;
use crate::utils::Info;
use crate::utils::info;

pub fn init(){
    let cur_path = "./";
    for dir in [utils::PAGE_DIR, utils::PUBLIC_DIR, utils::SOURCE_DIR, utils::STATIC_DIR, utils::TEMPLATE_DIR]{
        let dir_path =format!("{}/{}", cur_path, dir);
        info(Info::INIT, "create", &dir_path);
        let dir_path = path::Path::new(&dir_path);
        fs::create_dir_all(dir_path).unwrap();
    }
    init_config();
}

fn init_config(){
    info(Info::INIT, "create", utils::CONFIG_FILE);
    let contents = 
r#"# config.toml
# manager your blog settings

# theme
theme="default"
"#;
    fs::write(path::Path::new(utils::CONFIG_FILE), contents).unwrap()
}