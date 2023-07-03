use std::path::PathBuf;

fn main() {
    println!("this works.");
    megadep::process(PathBuf::from("./crates/alpha/src/lib.rs"));
    println!("this fails, because of the dependency on `alpha`.");
    megadep::process(PathBuf::from("./crates/beta/src/lib.rs"));
}
