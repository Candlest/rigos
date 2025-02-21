use crate::{render, PUBLIC_PATH};
use log::{error, info};
use notify::{Error, Event, RecursiveMode, Watcher};
use std::path::Path;
use tokio::task;
use warp::Filter;

pub(crate) async fn preview(watch: bool) {
    let path = PUBLIC_PATH.to_str().unwrap();
    if !watch {
        // 如果不需要监视文件变动，仅启动服务器
        let files = warp::fs::dir(path);
        let app = warp::get().and(files);
        let server = warp::serve(app).run(([127, 0, 0, 1], 8080));
        let _ = server.await;
        return;
    }

    println!("watchdog is running...");

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, Error>| {
        if let Ok(event) = res {
            match event.kind {
                notify::EventKind::Modify(_)
                    if !event
                        .paths
                        .iter()
                        .any(|p| p.to_string_lossy().contains(path)) =>
                {
                    if tx.send(event.clone()).is_err() {
                        error!("Error sending event through channel");
                    }
                }
                _ => {}
            }
            info!("event: {:?}", event);
        } else {
            error!("watch error: {:?}", res.err().unwrap());
        }
    })
    .expect("watcher error");

    watcher
        .watch(Path::new("."), RecursiveMode::Recursive)
        .expect("watch error");
    let files = warp::fs::dir(path);
    let app = warp::get().and(files);
    let server_future = warp::serve(app).run(([127, 0, 0, 1], 8080));
    let render_task = task::spawn(async move {
        while let Ok(event) = rx.recv() {
            if !event
                .paths
                .iter()
                .any(|p| p.to_string_lossy().contains(path))
            {
                task::spawn(async {
                    render::render_all();
                });
            }
        }
    });
    let _ = tokio::join!(server_future, render_task);
}
