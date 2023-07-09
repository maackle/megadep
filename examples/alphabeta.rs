use std::path::PathBuf;

use megadep::DefGraph;

fn main() {
    let m = megadep::Megadep {
        deps_dir: "./examples/crates/target/debug/deps".to_string(),
    };

    let graph = [
        m.process("alpha", PathBuf::from("./examples/crates/alpha/src/lib.rs")),
        m.process("beta", PathBuf::from("./examples/crates/beta/src/lib.rs")),
    ]
    .into_iter()
    .fold(DefGraph::default(), |a, m| a.extend(m));

    dbg!(graph);
}
