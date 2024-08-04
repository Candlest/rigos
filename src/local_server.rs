use notify::{Error, Event, RecursiveMode, Watcher};
use std::path::Path;
use tokio::task;
use warp::Filter;
use crate::{io::info, render};

pub(crate) async fn preview(watch: bool) {
    if !watch {
        // 如果不需要监视文件变动，仅启动服务器
        let files = warp::fs::dir("pub");
        let app = warp::get().and(files);
        let server = warp::serve(app).run(([127, 0, 0, 1], 8080));
        let _ = server.await;
        return;
    }

    info("Watching for changes...");

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::recommended_watcher(move |res: Result<Event, Error>| {
        if let Ok(event) = res {
            match event.kind {
                // 检查是否为除 pub 文件夹以外的文件修改事件
                notify::EventKind::Modify(_)
                    if !event.paths.iter().any(|p| p.to_string_lossy().contains("/pub/")) =>
                {
                    // 发送事件到通道，以便在另一个任务中处理
                    if tx.send(event.clone()).is_err() {
                        eprintln!("Error sending event through channel");
                    }
                }
                _ => {}
            }
            //println!("event: {:?}", event);
        } else {
            //println!("watch error: {:?}", res.err().unwrap());
        }
    })
    .expect("watcher error");

    watcher
        .watch(Path::new("."), RecursiveMode::Recursive)
        .expect("watch error");

    // 启动 warp 服务器
    let files = warp::fs::dir("pub");
    let app = warp::get().and(files);
    let server_future = warp::serve(app).run(([127, 0, 0, 1], 8080));

    // 异步任务用于处理文件变动
    let render_task = task::spawn(async move {
        while let Ok(event) = rx.recv() {
            // 这里我们只处理非 pub 文件夹的事件
            if !event.paths.iter().any(|p| p.to_string_lossy().contains("/pub/")) {
                task::spawn(render::render());
                //info("Updated");
            }
        }
    });

    // 运行服务器和文件监视任务
    let _ = tokio::join!(server_future, render_task);
}