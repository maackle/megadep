use std::path::PathBuf;

fn main() {
    let m = megadep::Megadep {
        deps_dir: "./target/debug/deps".to_string(),
    };
    m.process(PathBuf::from("./crates/beta/src/lib.rs"));
}
