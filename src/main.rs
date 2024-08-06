mod config;
mod create;
mod deploy;
mod io;
mod local_server;
mod render;

use clap::{arg, Parser, Subcommand};
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
    /// Create a new item with type and name
    #[clap(visible_alias("n"))]
    New {
        #[arg(required = true, help = "The type of the new item (e.g., post, page)")]
        type_: String,

        #[arg(required = true, help = "The name of the new item")]
        name: String,
    },
    /// Render html site from templates & sources
    #[clap(visible_alias("r"))]
    Render,
    /// Preview the static site from localhost
    #[clap(visible_alias("p"))]
    Preview {
        #[arg(
            short = 'w',
            long = "watch",
            help = "Enable watch mode to automatically refresh on changes"
        )]
        watch: bool,
    },
    /// Render & preview
    #[clap(visible_alias("x"))]
    Rap {
        #[arg(
            short = 'w',
            long = "watch",
            help = "Enable watch mode to automatically refresh on changes"
        )]
        watch: bool,
    },
    /// Deploy the static site to remote
    #[clap(visible_alias("d"))]
    Deploy,
    /// Init a new site
    #[clap(visible_alias("i"))]
    Init,
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    // args
    let cli = Cli::parse(); // get cli
    match &cli.command {
        Some(Commands::Render) => {
            io::info("rendering...");
            render::render().await;
        }
        Some(Commands::Preview{watch}) => {
            io::info("preview at http://localhost:8080");
            io::info("you can exit with CTRL + C");
            let _ = local_server::preview(*watch).await;
        }
        Some(Commands::Rap{watch}) => {
            io::info("rendering & previewing...");
            render::render().await;
            io::info("preview at http://localhost:8080");
            io::info("you can exit with CTRL + C");
            let _ = local_server::preview(*watch).await;
        }
        Some(Commands::Deploy) => {
            io::info("deploying to remote...");
            deploy::deploy();
        }
        Some(Commands::Init) => {
            io::info("init new site at current directory...");
            create::create_new_site();
        }
        Some(Commands::New { type_, name }) => {
            match type_.as_str() {
                "post" => create::create_new_post(name.to_string()),
                "page" => {
                    create::create_new_page(name.to_string());
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
