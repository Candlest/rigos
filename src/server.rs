use crate::{utils::{self, info}, builder::Builder};
/*Listen TCP */
pub async fn run_server() {
    info(utils::Info::RUN, "run at", "http://localhost:7878");
    info(utils::Info::RUN, "you can exit with", "CTRL + C");
    let pro_path = String::from("./");
    let builder = Builder::new(pro_path);
    let path = builder.config.public_dir;
    let _port = std::env::args()
        .nth(2)
        .unwrap_or("7878".into())
        .parse::<u16>()
        .unwrap_or(7878_u16);
    let api = warp::fs::dir(path);
    let server = warp::serve(api);
    server.run(([0, 0, 0, 0], 7878)).await;
}
