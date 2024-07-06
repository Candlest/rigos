use crate::{config, io};

pub fn crate_new_page(name : String){
    let _ = io::write_to_file(&format!("./{}.md", name), "# New Page");
    // 注册
    let mut cfg = config::read_config("config.toml").unwrap();
    cfg.pages.push(name);
    config::write_config("config.toml", &cfg).unwrap();
    io::info("new page created");
}

pub fn crate_new_post(opath : String){
    let (path, filename) = io::separate_path_and_filename(&opath.clone());
    let filename = filename.unwrap();
    let contents = format!("
title=\"{}\"
filename=\"{}\"
date=\"{}\"
tags=[]
category=\"\"
%%%%%%
", filename, filename, path.unwrap().to_str().unwrap());
    let _ = io::write_to_file(&format!("./{}.md", opath), &contents);
    io::info("new post created");
}