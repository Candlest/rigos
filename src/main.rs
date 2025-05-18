mod conf;
mod render;
mod deploy;

use std::path::PathBuf;

use anyhow::{self, Context};
use colored::Colorize;
use log::{self};
use once_cell::sync::Lazy;
mod local_server;
static PATH: Lazy<PathBuf> =
    Lazy::new(|| std::env::current_dir().expect("Failed to get current directory"));
static PAGE_PATH: Lazy<PathBuf> = Lazy::new(|| PATH.join("pages"));
static POST_PATH: Lazy<PathBuf> = Lazy::new(|| PATH.join("posts"));
static PUBLIC_PATH: Lazy<PathBuf> = Lazy::new(|| PATH.join("public"));
static ASSETS_PATH: Lazy<PathBuf> = Lazy::new(|| PATH.join("assets"));
static CONFIG: Lazy<conf::RigosConfig> = Lazy::new(|| {
    let path = PATH.join("config.toml");
    let config = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config file: {:?}", path))
        .expect("Failed to read config file");
    toml::from_str(&config).expect("Failed to parse config file")
});

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
    env_logger::init();
    log::set_max_level(log::LevelFilter::Info);
    let start = Instant::now();
    let cli = Cli::parse(); // get cli

    if let Some(command) = &cli.command {
        handle_command(command).await;
    } else {
        println!(
            "please input subcommand or use {} to get more information...",
            "rigos help".bold().blue()
        );
    }

    let duration = start.elapsed();
    println!(
        "Exit, with {} seconds",
        duration.as_secs_f32().to_string().bold().blue()
    );
}

async fn handle_command(command: &Commands) {
    match command {
        Commands::Render => handle_render().await,
        Commands::Preview { watch } => handle_preview(*watch).await,
        Commands::Rap { watch } => handle_rap(*watch).await,
        Commands::Deploy => handle_deploy(),
        Commands::Init => handle_init(),
    }
}

async fn handle_render() {
    println!("rendering...");
    render::render_all();
}

async fn handle_preview(watch: bool) {
    println!("preview at {}", "http://localhost:8080".bold().blue());
    println!("you can exit with {}", "CTRL + C".bold().blue());
    let _ = local_server::preview(watch).await;
}

async fn handle_rap(watch: bool) {
    println!("rendering & previewing...");
    handle_render().await;
    println!("preview at {}", "http://localhost:8080".bold().blue());
    println!("you can exit with {}", "CTRL + C".bold().blue());
    let _ = local_server::preview(watch).await;
}

fn handle_deploy() {
    println!("deploying to remote...");
    deploy::deploy();
}

fn handle_init() {
    println!("init new site at current directory...");
    //create::create_new_site();
}
