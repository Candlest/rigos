
use crate::{
    builder::Builder,
};

use super::*;
/*Print Help List */
// pub fn help() {
//     /*Help List*/
//     let text: &'static str = "
//         rublog - A static blog generator powered by Rust
//         ------------------------------------------------
//         help    : show help
//         build   : generator site files from markdown
//         clear   : clear generated files
//         run     : run this blog site on local server
//         cbr     : clear, build and run
//         ------------------------------------------------
//         Get more information from https://github.com/Candlest/rublog
//         ";
//     println!("{}", text.green());
// }

/*Clear PUBLIC_DIR */
pub fn clear() {
    let pro_path = String::from("./");
    let builder = Builder::new(pro_path);
    builder.clear_post();
}

/*Generate PUBLIC_DIR*/
pub fn build() {
    let pro_path = String::from("./");
    let builder = Builder::new(pro_path);
    let mut builder_o = builder.clone();
    utils::info(utils::Info::GENERATE, "now generate", utils::PUBLIC_DIR);
    let handle_static = std::thread::spawn(move || {
        builder.build_static_dir();
    });
    builder_o.pre_create_posts_index();
    builder_o.build_all();
    handle_static.join().unwrap();
}

pub fn check(){
    let pro_path = String::from("./");
    let mut builder = Builder::new(pro_path);
    builder.check_pub();
}

/*RUN PUBLIC_DIR on local web server*/
pub async fn run() {
    //utils::info(utils::Info::RUN, "now run", utils::PUBLIC_DIR);
    server::run_server().await;
}

pub fn init() {
    utils::info(utils::Info::INIT, "now init", "./");
    init::init();
}
