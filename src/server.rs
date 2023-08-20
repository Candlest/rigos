use crate::{utils::{self, info}, builder::Builder};
/*Listen TCP */
pub async fn run_server() {
    info(utils::Info::RUN, "run at", "http://localhost:7878");
    info(utils::Info::RUN, "you can exit with", "CTRL + C");

    /*
     * 解决Route
     */
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
    // let mut server = web_server::new()
    //     .not_found(Box::new(|req, _| {
    //     info(utils::Info::RUN, "cannot find", req.get_path().as_str());
    //     std::path::Path::new(format!("{}/404.html", PUBLIC_DIR).as_str()).into()
    // }));

    // for item in builder.pub_folders {
    //     let item = &item[7..];
    //     server.get(format!("/{}/", item).as_str(), Box::new(|request, _| {
    //         info(
    //             utils::Info::RUN,
    //             "require file",
    //             request.get_path().as_str(),
    //         );
    //         std::path::Path::new(format!("{}{}", PUBLIC_DIR, request.get_path()).as_str())
    //             .into()
    //     }));
    // }

    // web_server::new()
    //     .not_found(Box::new(|req, _| {
    //         info(utils::Info::RUN, "cannot find", req.get_path().as_str());
    //         std::path::Path::new(format!("{}/404.html", PUBLIC_DIR).as_str()).into()
    //     }))
    //     .get(
    //         "/",
    //         Box::new(|_, _| {
    //             std::path::Path::new(format!("{}/index.html", PUBLIC_DIR).as_str()).into()
    //         }),
    //     )
    //     .get(
    //         "/:f",
    //         Box::new(|request, _| {
    //             info(
    //                 utils::Info::RUN,
    //                 "require file",
    //                 request.get_path().as_str(),
    //             );
    //             std::path::Path::new(format!("{}{}", PUBLIC_DIR, request.get_path()).as_str())
    //                 .into()
    //         }),
    //     )
    //     .get(
    //         "/Post/:f",
    //         Box::new(|request, _| {
    //             let decoded = percent_decode(request.get_path().as_str().as_bytes())
    //                 .decode_utf8()
    //                 .unwrap()
    //                 .to_string();
    //             info(utils::Info::RUN, "require file", &decoded);
    //             std::path::Path::new(&format!("{}{}", PUBLIC_DIR, decoded)).into()
    //         }),
    //     )
    //     .launch(7878);
}
