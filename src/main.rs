use std::path::PathBuf;

use rustc_errors::registry::Registry;
use rustc_interface::Config;
use rustc_session::config::Input;

fn main() {
    let path = PathBuf::from("/home/michael/Holo/chain/crates/mr_bundle/lib.rs");
    let input = Input::File(path);
    let config = Config {
        input,
        registry: Registry::new(&[]),
        // defaults
        opts: Default::default(),
        crate_cfg: Default::default(),
        crate_check_cfg: Default::default(),
        output_dir: Default::default(),
        output_file: Default::default(),
        file_loader: Default::default(),
        locale_resources: Default::default(),
        lint_caps: Default::default(),
        parse_sess_created: Default::default(),
        register_lints: Default::default(),
        override_queries: Default::default(),
        make_codegen_backend: Default::default(),
    };
    rustc_interface::run_compiler(config, |_compiler| println!("hello from rustc!"))
}
