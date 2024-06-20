mod config;
mod deploy;
mod io;
mod local_server;
mod render;

use clap::{arg, Arg, Command, Parser, Subcommand};
use deploy::deploy;
use std::time::Instant; // counting // args

#[derive(Parser)]
#[command(
    name = "rigos",
    about = "Rigos is a generator of sites, written with Rust",
    version = "1.0"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// render html site from templates & sources
    render,
    /// preview the static site from localhost
    preview,
    /// render & preview
    rap,
    /// deploy the static site to remote
    deploy,
}

#[actix_web::main]
async fn main() {
    let start = Instant::now();
    // args
    let cli = Cli::parse(); // get cli
    match &cli.command {
        Some(Commands::render) => {
            io::info("rendering...");
            render::render();
        }
        Some(Commands::preview) => {
            io::info("preview at http://localhost:8080");
            io::info("you can exit with CTRL + C");
            let _ = local_server::preview().await;
        }
        Some(Commands::rap) => {
            io::info("rendering & previewing...");
            render::render();
            io::info("preview at http://localhost:8080");
            io::info("you can exit with CTRL + C");
            let _ = local_server::preview().await;
        }
        Some(Commands::deploy) => {
            io::info("deploying...");
            deploy::deploy();
        }
        None => io::info("please input subcommand or use `rigos help` to get more information..."),
    }

    let duration = start.elapsed();
    io::info(&format!("Exit, with {} seconds", duration.as_secs_f32()));
}
