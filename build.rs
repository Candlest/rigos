use copy_to_output::copy_to_output;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=site/*");
    copy_to_output("site", &env::var("PROFILE").unwrap()).expect("Could not copy");
}
