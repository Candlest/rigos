
use crate::{
    builder::Builder, utils, server, init,
};

/*Clear PUBLIC_DIR */
pub fn clear() {
    let pro_path = String::from("./");
    let mut builder = Builder::new(pro_path);
    builder.check_pub();
    builder.clear();
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
    builder_o.build_feed();
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
