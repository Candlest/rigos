use clap::{Parser, Subcommand};
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

    // Continued program logic goes here...
}

// use clap::{App, Arg, SubCommand};

// use utils::info;

// mod build;
// mod clear;
// mod generate;
// mod init;
// mod options;
// mod server;
// mod utils;
// fn main() {
//     let cmd = clap::Command::new("cargo")
//         .bin_name("cargo")
//         .subcommand_required(true)
//         .subcommand(
//             clap::command!("example").arg(
//                 clap::arg!(--"manifest-path" <PATH>)
//                     .value_parser(clap::value_parser!(std::path::PathBuf)),
//             ),
//         );
//     let matches = cmd.get_matches();
//     let matches = match matches.subcommand() {
//         Some(("example", matches)) => matches,
//         _ => unreachable!("clap should ensure we don't get here"),
//     };
//     let manifest_path = matches.get_one::<std::path::PathBuf>("manifest-path");
//     println!("{manifest_path:?}");

//     // let arg = std::env::args();
//     // match arg.len() {
//     //     1 => options::help(),
//     //     2 => match arg.last().unwrap().as_str() {
//     //         "help" => options::help(),
//     //         "clear" => options::clear(),
//     //         "build" => options::build(),
//     //         "run" => options::run(),
//     //         "init" => options::init(),
//     //         "cbr" => {
//     //             options::clear();
//     //             let handle_build = std::thread::spawn(|| {
//     //                 options::build();
//     //             });
//     //             handle_build.join().unwrap();
//     //             options::run();
//     //         }
//     //         _ => options::help(),
//     //     },
//     //     3 => {
//     //         /*post add, del, rename
//     //          *page add, del
//     //          *init
//     //          */
//     //     }
//     //     _ => {
//     //         info(utils::Info::ERROR, "unexpected args", "");
//     //     }
//     // }
// }
