use serde::{Deserialize, Serialize};
use toml::value::Datetime;

use crate::utils::{self, PostObject, Post};

/* generate.rs
 * 使用它替代build.rs
 */

pub struct PostGenerator {
    file_path: String,
    theme_dir: String,
    toml_info: String,
    body_html: String,
    html_path: String,
}

impl PostGenerator {
    pub fn new(path: String, theme: String) -> PostGenerator {
        PostGenerator {
            file_path: path,
            theme_dir: theme,
            toml_info: "".to_string(),
            body_html: "".to_string(),
            html_path: "".to_string(),
        }
    }
    fn split_file(&mut self) {
        //println!("{}", md_file);
        let md_path = &self.file_path;
        let raw_text = std::fs::read_to_string(md_path).unwrap();
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
        self.toml_info = toml_t.to_string();
        self.body_html = body;
    }
    pub fn build(&mut self) {
        (self.toml_info, self.body_html) = utils::read_markdown(&self.file_path);
        self.split_file(); /*Get toml, html body */
        let tera = match tera::Tera::new(
            format!("{}/{}/**/*.html", utils::TEMPLATE_DIR, self.theme_dir).as_str(),
        ) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        let mut context = tera::Context::new();
        let rendered;
        context.insert("body", self.body_html.as_str());

        let file_name = std::path::Path::new(&self.file_path);
        let file_name = file_name.file_stem().unwrap().to_str().unwrap(); //UNWARP

        self.html_path = format!("{}/Post/{}.html", utils::PUBLIC_DIR, file_name);
        let post: Post = toml::from_str(&self.toml_info).unwrap();
        context.insert("post", &post);
        //render
        rendered = tera.render("post.html", &context).unwrap();

        let folder = std::path::Path::new(&self.html_path).parent().unwrap();
        let _ = std::fs::create_dir_all(folder);
        let htm_path = &self.html_path;
        std::fs::write(htm_path, rendered).unwrap();
    }
    pub fn get_obejct(&mut self) -> PostObject {
        let p: Post = toml::from_str(&self.toml_info).unwrap();
        let web_path = utils::path_local2web(&self.html_path);
        PostObject {
            title: p.title,
            datetime: p.datetime,
            tags: p.tags,
            category: p.category,
            url: web_path,
        }
    }
}
