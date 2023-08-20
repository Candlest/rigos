use colored::Colorize;
use serde::{Deserialize, Serialize};
use toml::value::Datetime;

/* fn info: Output content in a canonical manner*/
pub fn info(inforation: Info, content: &str, remark: &str) {
    let color_str = match inforation {
        Info::GENERATE => ("[GENERATE]".bold().blue(), remark.green()),
        Info::CLEAR => ("[CLEAR]".bold().blue(), remark.green()),
        Info::RUN => ("[RUN]".bold().blue(), remark.green()),
        Info::ERROR => ("[ERROR]".bold().red(), remark.red()),
        Info::INIT => ("[INIT]".bold().blue(), remark.green()),
    };
    println!("{}{:20}\t\t{}", color_str.0, content, color_str.1);
}
pub enum Info {
    GENERATE,
    RUN,
    CLEAR,
    ERROR,
    INIT,
}

/* Define DIR as constant*/
pub const PUBLIC_DIR: &str = "public";
pub const SOURCE_DIR: &str = "source";
pub const STATIC_DIR: &str = "static";
pub const PAGE_DIR: &str = "page";
pub const TEMPLATE_DIR: &str = "template";

/*config */
pub const CONFIG_FILE: &str = "./config.toml";

/* Thanks to web_server
 * We never use percent_encode again :-)
 */
pub fn path_local2web(local_path: &str) -> String {
    local_path[PUBLIC_DIR.len()..].to_string() /*非常方便！！！ */
}

pub fn path_root2pub(raw_path: &str) -> (String, String) {
    let path = std::path::Path::new(raw_path);
    let filename = path.file_name().unwrap();
    let result = &raw_path[PUBLIC_DIR.len()..raw_path.len() - &filename.len()]; /*非常方便！！！ */
    (
        format!("{}{}{}", PUBLIC_DIR, result, filename.to_str().unwrap()),
        format!("{}{}", PUBLIC_DIR, result),
    ) /*/css/xxx.js */
}

pub(crate) fn get_path_list(path: &str) -> Vec<String> {
    let mut my_filename_list: Vec<String> = vec![];
    // 只需要文件及对应的路径，不需要空文件夹的名称及路径
    for e in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            //println!("{}", e.path().display());
            my_filename_list.push(e.path().display().to_string());
        } else {
            crate::utils::info(
                crate::utils::Info::GENERATE,
                "found dectory",
                e.path().display().to_string().as_str(),
            );
        }
    }
    my_filename_list
}

pub(crate) fn get_folder_list(path: &str) -> Vec<String> {
    let mut my_filename_list: Vec<String> = vec![];
    // 只需要文件夹的名称及路径
    for e in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if !e.metadata().unwrap().is_file() && !e.path().display().to_string().contains(".git") {
            crate::utils::info(
                crate::utils::Info::GENERATE,
                "found dectory",
                e.path().display().to_string().as_str(),
            );
            my_filename_list.push(e.path().display().to_string());
        }
    }
    my_filename_list
}

/* md 2 html
 * 我们只生成toml, body
*/
pub fn read_markdown(md_file: &str) -> (String, String) {
    //println!("{}", md_file);
    let raw_text = std::fs::read_to_string(md_file).expect(md_file);
    let cut_raw: Vec<&str> = raw_text.split("---").collect();
    let toml_text = cut_raw[1];
    let toml_t = toml_text.clone();
    /* TOML is OK */
    let md_raw = &cut_raw[2..];
    let mut md_text: String = "".to_string();
    for md in md_raw {
        md_text.push_str(&md);
    }
    let parser = pulldown_cmark::Parser::new_ext(&md_text, pulldown_cmark::Options::all());
    let mut body = String::new();
    pulldown_cmark::html::push_html(&mut body, parser);
    /*BODY is OK */
    (toml_t.to_string(), body)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub title: String,
    pub datetime: Datetime,
    pub tags: Vec<String>,
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostString {
    pub title: String,
    pub datetime: String,
    pub tags: Vec<String>,
    pub category: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostObject {
    pub title: String,
    pub datetime: String,
    pub tags: Vec<String>,
    pub category: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub page_templates: Vec<String>,
    pub public_dir: String,
    pub source_dir: String,
    pub static_dir: String,
    pub page_dir: String,
    pub template_dir: String,
    pub theme: String,
}

impl Config {
    pub fn new(path: String) -> Config {
        let cfg_cont = std::fs::read_to_string(path).unwrap();
        toml::from_str(cfg_cont.as_str()).expect("config file can not read")
    }
}