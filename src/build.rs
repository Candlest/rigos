use serde::{Deserialize, Serialize};
use std::ops::Add;
use toml::{Value, value::Datetime};
use crate::utils::{self, PostObject, Post, get_path_list};

pub fn build_pages() {
    let getfilelist: Vec<String> = get_path_list(utils::PAGE_DIR);
    for p in getfilelist.iter() {
        if p.ends_with("markdown") || p.ends_with("md") {
            utils::info(utils::Info::GENERATE, "found", p);

            let htfile = std::path::Path::new(p);
            let htfile = htfile.file_stem().unwrap().to_str().unwrap(); //UNWARP
            let html_file = format!("{}/{}.html", utils::PUBLIC_DIR, htfile);
            utils::info(utils::Info::GENERATE, "creating", &html_file);
            //add index
            let web_path = utils::path_local2web(&html_file);
            //end add
            let (toml_t, body_h) = read_markdown(p);
            create_html_from_page(html_file, toml_t, body_h);
        }
    }
}

/*Static DIR */
pub fn build_static_dir() {
    /*Static DIR */
    let getfilelist: Vec<String> = get_path_list(utils::STATIC_DIR);
    for p in getfilelist.iter() {
        utils::info(utils::Info::GENERATE, "copying", &p);
        let stfile_path = std::path::Path::new(p);
        let (st_file, st_file_dir) = utils::path_root2pub(&p);
        std::fs::create_dir_all(std::path::Path::new(st_file_dir.as_str())).unwrap(); //UNWRAP
        match std::fs::copy(stfile_path, std::path::Path::new(st_file.as_str())) {
            Ok(_) => utils::info(utils::Info::GENERATE, "copied", &p),
            Err(_) => utils::info(utils::Info::GENERATE, "failed to copy", ""),
        }
    }
}

// pub fn build_posts_and_index() {
//     /*Foreach */
//     let getfilelist: Vec<String> = get_path_list(utils::SOURCE_DIR);
//     let mut posts_vec: Vec<Post_Url> = vec![];
//     for p in getfilelist.iter() {
//         if p.ends_with("markdown") || p.ends_with("md") {
//             utils::info(utils::Info::GENERATE, "found", p);

//             let htfile = std::path::Path::new(p);
//             let htfile = htfile.file_stem().unwrap().to_str().unwrap(); //UNWARP
//             let html_file = format!("{}/Post/{}.html", utils::PUBLIC_DIR, htfile);
//             utils::info(utils::Info::GENERATE, "creating", &html_file);
//             //add index
//             let web_path = utils::path_local2web(&html_file);
//             //println!("{}", web_path);
//             posts_vec.push(Post_Url {
//                 name: htfile.to_string(),
//                 url: web_path,
//             });
//             //end add
//             let (toml_t, body_h) = read_markdown(p);
//             create_html_from_post(html_file, toml_t, body_h);
//         }
//     }
//     let (toml_t, body_h) = read_markdown(format!("{}/{}", utils::PAGE_DIR, "index.md").as_str());
//     create_html_from_index(toml_t, posts_vec, body_h);
// }

//----------------------------------------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------------------------------------

// pub fn create_html_from_post(html_path: String, toml_info: String, body_html: String) {
//     let tera = match tera::Tera::new(format!("{}/**/*.html", utils::TEMPLATE_DIR).as_str()) {
//         Ok(t) => t,
//         Err(e) => {
//             println!("Parsing error(s): {}", e);
//             ::std::process::exit(1);
//         }
//     };
//     let mut context = tera::Context::new();
//     let post: Post = toml::from_str(&toml_info).unwrap();
//     //dbg!(post);
//     context.insert("body", body_html.as_str());
//     context.insert("post", &post);
//     //render
//     let rendered = tera.render("post.html", &context).unwrap();
//     let folder = std::path::Path::new(&html_path).parent().unwrap();
//     let _ = std::fs::create_dir_all(folder);
//     std::fs::write(html_path, rendered);
// }

pub fn create_html_from_page(html_path: String, toml_info: String, body_html: String) {
    let tera = match tera::Tera::new(format!("{}/**/*.html", utils::TEMPLATE_DIR).as_str()) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let mut context = tera::Context::new();
    let page: Page = toml::from_str(&toml_info).unwrap();
    //dbg!(post);
    context.insert("body", body_html.as_str());
    context.insert("page", &page);
    //render
    let rendered = tera.render("page.html", &context).unwrap();
    let folder = std::path::Path::new(&html_path).parent().unwrap();
    let _ = std::fs::create_dir_all(folder);
    std::fs::write(html_path, rendered).unwrap();
}

pub fn create_index(post_urls: Vec<PostObject>, body_html: String) {
    let html_path = format!("{}/{}", utils::PUBLIC_DIR, "index.html");
    let tera = match tera::Tera::new(format!("{}/**/*.html", utils::TEMPLATE_DIR).as_str()) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let mut context = tera::Context::new();
    //dbg!(post);
    context.insert("body", body_html.as_str());
    //dbg!(post_urls);
    context.insert("post_index", &post_urls);
    //render
    let rendered = tera.render("index.html", &context).unwrap();
    //println!("{}", rendered);
    let folder = std::path::Path::new(&html_path).parent().unwrap();
    let _ = std::fs::create_dir_all(folder);
    crate::utils::info(
        crate::utils::Info::GENERATE,
        "generating index of post",
        &html_path,
    );
    std::fs::write(html_path, rendered).unwrap();
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    title: String,
}

