use clap::{Parser, Subcommand};
use std::time::{Duration, Instant};
use rublog::options;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate the static site
    Build,
    /// Run a local web server
    Run,
    /// Build & Run all in one
    BR,
    /// Check
    Check,
    /// Clear public/Post/
    Clear
}

#[tokio::main]
async fn main(){
    let start = Instant::now();
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Build) =>{
            options::build();
        },
        Some(Commands::Run) =>{
            options::run().await;
        },
        Some(Commands::BR) => {
            options::build();
            options::run().await;
        }
        Some(Commands::Clear) => {
            options::clear();
        },
        Some(Commands::Check) => {
            options::check();
        },
        None => {}
    }

    let duration = start.elapsed();

    println!("Time elapsed: {:?}", duration);
}