use colored::Colorize;

use crate::{
    build::create_index,
    utils::{get_path_list, read_markdown, PostObject, CONFIG_FILE, PAGE_DIR, STATIC_DIR},
};

use super::*;
/*Print Help List */
pub fn help() {
    /*Help List*/
    let text: &'static str = "
        rublog - A static blog generator powered by Rust
        ------------------------------------------------
        help    : show help
        build   : generator site files from markdown
        clear   : clear generated files
        run     : run this blog site on local server
        cbr     : clear, build and run
        ------------------------------------------------
        Get more information from https://github.com/Candlest/rublog
        ";
    println!("{}", text.green());
}

/*Clear PUBLIC_DIR */
pub fn clear() {
    utils::info(utils::Info::CLEAR, "now clear", utils::PUBLIC_DIR);
    clear::clear_all();
}

/*Generate PUBLIC_DIR*/
pub fn build() {
    let cfg_cont = std::fs::read_to_string(CONFIG_FILE).unwrap();
    let cfg: utils::Config = toml::from_str(cfg_cont.as_str()).unwrap();
    let theme_name = cfg.theme;

    utils::info(utils::Info::GENERATE, "now generate", utils::PUBLIC_DIR);
    let handle_static = std::thread::spawn(|| {
        build::build_static_dir();
    });

    build::build_pages(theme_name.clone());
    handle_static.join().unwrap();

    let getfilelist: Vec<String> = get_path_list(utils::SOURCE_DIR);
    let mut posts_vec: Vec<PostObject> = vec![];
    for p in getfilelist.iter() {
        if p.ends_with("markdown") || p.ends_with("md") {
            utils::info(utils::Info::GENERATE, "found", p);
            let mut post_item = generate::PostGenerator::new(String::from(p), theme_name.clone());
            post_item.build();
            posts_vec.push(post_item.get_obejct());
        }
    }
    let index_body_html = read_markdown(format!("{}/index.md", PAGE_DIR).as_str()).1;
    create_index("index.html".to_string(),posts_vec.clone(), index_body_html, theme_name.clone());

    let index_body_html = read_markdown(format!("{}/tags.md", PAGE_DIR).as_str()).1;
    create_index("tags.html".to_string(),posts_vec.clone(), index_body_html, theme_name.clone());

    let index_body_html = read_markdown(format!("{}/categories.md", PAGE_DIR).as_str()).1;
    create_index("categories.html".to_string(),posts_vec, index_body_html, theme_name.clone());
}

/*RUN PUBLIC_DIR on local web server*/
pub fn run() {
    utils::info(utils::Info::RUN, "now run", utils::PUBLIC_DIR);
    server::run_server();
}

pub fn init() {
    utils::info(utils::Info::INIT, "now init", "./");
    init::init();
}
//-------------
