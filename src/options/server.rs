use percent_encoding::percent_decode;
use web_server;

use crate::utils::{self, info, PUBLIC_DIR};
/*Listen TCP */
pub fn run_server() {
    info(utils::Info::RUN, "run at", "http://localhost:7878");
    info(utils::Info::RUN, "you can exit with", "CTRL + C");

    /*
     * 目前仍然没有解决Route
     */
    web_server::new()
        .not_found(Box::new(|req, _| {
            info(utils::Info::RUN, "cannot find", req.get_path().as_str());
            std::path::Path::new(format!("{}/404.html", PUBLIC_DIR).as_str()).into()
        }))
        .get(
            "/",
            Box::new(|_, _| {
                std::path::Path::new(format!("{}/index.html", PUBLIC_DIR).as_str()).into()
            }),
        )
        .get(
            "/:f",
            Box::new(|request, _| {
                info(
                    utils::Info::RUN,
                    "require file",
                    request.get_path().as_str(),
                );
                std::path::Path::new(format!("{}{}", PUBLIC_DIR, request.get_path()).as_str())
                    .into()
            }),
        )
        .get(
            "/Post/:f",
            Box::new(|request, _| {
                let decoded = percent_decode(request.get_path().as_str().as_bytes())
                    .decode_utf8()
                    .unwrap()
                    .to_string();
                info(utils::Info::RUN, "require file", &decoded);
                std::path::Path::new(&format!("{}{}", PUBLIC_DIR, decoded)).into()
            }),
        )
        // .get("/Resource:f", Box::new(|request, _| {
        //     info(utils::Info::RUN, "require file", request.get_path().as_str());
        //     std::path::Path::new(format!("{}{}", PUBLIC_DIR, request.get_path()).as_str()).into()
        // }))
        .launch(7878);
}
