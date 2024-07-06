mod config;
mod deploy;
mod io;
mod local_server;
mod render;
mod create;

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
    /// create posts & pages
    #[command(name = "new")]
    New {
        #[arg(short = 't', long = "type", help = "The type of the new item (e.g., post, page)")]
        type_: String, // 使用 type_ 来避免与 Rust 的关键字 type 冲突

        #[arg(short = 'n', long = "name", help = "The name of the new item")]
        name: String,
    },
    /// render html site from templates & sources
    Render,
    /// preview the static site from localhost
    Preview,
    /// render & preview
    Rap,
    /// deploy the static site to remote
    Deploy,
}

#[actix_web::main]
async fn main() {
    let start = Instant::now();
    // args
    let cli = Cli::parse(); // get cli
    match &cli.command {
        Some(Commands::Render) => {
            io::info("rendering...");
            render::render();
        }
        Some(Commands::Preview) => {
            io::info("preview at http://localhost:8080");
            io::info("you can exit with CTRL + C");
            let _ = local_server::preview().await;
        }
        Some(Commands::Rap) => {
            io::info("rendering & previewing...");
            render::render();
            io::info("preview at http://localhost:8080");
            io::info("you can exit with CTRL + C");
            let _ = local_server::preview().await;
        }
        Some(Commands::Deploy) => {
            io::info("deploying to remote...");
            deploy::deploy();
        }
        Some(Commands::New { type_, name }) => {
            match type_.as_str() {
                "post" => {
                    create::crate_new_post(name.to_string())
                }
                "page" =>{
                    create::crate_new_page(name.to_string());
                }
                _ => {
                    // 处理所有未明确匹配的情况
                    io::errstr("Error: Unsupported type");
                }
            }
        }
        None => io::info("please input subcommand or use `rigos help` to get more information..."),
    }

    let duration = start.elapsed();
    io::info(&format!("Exit, with {} seconds", duration.as_secs_f32()));
}
