mod options;
mod utils;
fn main() {
    let arg = std::env::args();
    match arg.last().unwrap().as_str() {
        "help" => options::help(),
        "clear" => options::clear(),
        "build" => options::build(),
        "run" => options::run(),
        "cbr" => {
            options::clear();
            let handle_build = std::thread::spawn(|| {
                options::build();
            });
            handle_build.join();
            options::run();
        }
        _ => options::help(),
    }
}
