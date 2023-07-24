use colored::Colorize;


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
    utils::info(utils::Info::GENERATE, "now generate", utils::PUBLIC_DIR);
    let handle_static = std::thread::spawn(|| {
        build::build_static_dir();
    });
    let handle_pages = std::thread::spawn(|| {
        build::build_pages();
    });
    handle_static.join().unwrap();
    handle_pages.join().unwrap();
    let handle_posts = std::thread::spawn(|| {
        build::build_posts_and_index();
    });
    handle_posts.join().unwrap();
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
