use actix_files as fs;
use actix_web::{App, HttpServer};

pub async fn preview() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(fs::Files::new("/", "./pub").index_file("index.html")))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
