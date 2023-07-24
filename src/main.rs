use utils::info;

mod options;
mod utils;
mod server;
mod build;
mod clear;
mod init;
mod generate;
fn main() {
    let arg = std::env::args();
    match arg.len() {
        1 => options::help(),
        2 => match arg.last().unwrap().as_str() {
            "help" => options::help(),
            "clear" => options::clear(),
            "build" => options::build(),
            "run" => options::run(),
            "init" => options::init(),
            "cbr" => {
                options::clear();
                let handle_build = std::thread::spawn(|| {
                    options::build();
                });
                handle_build.join().unwrap();
                options::run();
            }
            _ => options::help(),
        },
        3 => {
            /*post add, del, rename
             *page add, del
             *init
             */
        }
        _ => {
            info(utils::Info::ERROR, "unexpected args", "");
        }
    }
}
