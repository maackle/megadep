use std::path::PathBuf;

fn main() {
    let deps = vec![
        "./target/debug/deps".into(),
        // PathBuf::from("./crates/alpha"),
        // PathBuf::from("./crates/beta"),
    ];
    println!("this works.");
    megadep::process(PathBuf::from("./crates/alpha/src/lib.rs"), deps.clone());
    println!("this fails, because of the dependency on `alpha`.");
    megadep::process(PathBuf::from("./crates/beta/src/lib.rs"), deps.clone());
}
