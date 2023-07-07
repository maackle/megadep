use std::path::PathBuf;

fn main() {
    let m = megadep::Megadep {
        deps_dir: "./examples/crates/target/debug/deps".to_string(),
    };
    println!("ALPHA");
    m.process("alpha", PathBuf::from("./examples/crates/alpha/src/lib.rs"));
    println!("BETA");
    m.process("beta", PathBuf::from("./examples/crates/beta/src/lib.rs"));
}
