#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_session;
extern crate rustc_span;

use std::{
    path::{self, PathBuf},
    process, str,
};

use rustc_errors::registry;
use rustc_session::config::{self, CheckCfg};

// use rustc_hash::{FxHashMap, FxHashSet};
// use rustc_span::source_map;

pub fn process(path: PathBuf) {
    holochain_trace::test_run().ok();
    let out = process::Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .unwrap();
    let sysroot = str::from_utf8(&out.stdout).unwrap().trim();
    // let mut externs = BTreeMap::new();
    // externs.insert("beta".to_string(), ExternEntry)
    // let sysroot = "/home/michael/gitfork/rust/build/x86_64-unknown-linux-gnu/stage2";
    let config = rustc_interface::Config {
        // Command line options
        opts: config::Options {
            maybe_sysroot: Some(path::PathBuf::from(sysroot)),
            // externs: rustc_session::config::Externs::new(externs)
            ..config::Options::default()
        },
        // cfg! configuration in addition to the default ones
        crate_cfg: Default::default(), // FxHashSet<(String, Option<String>)>
        crate_check_cfg: CheckCfg::default(), // CheckCfg
        input: config::Input::File(path),
        // input: config::Input::Str {
        //     name: source_map::FileName::Custom("main.rs".into()),
        //     input: r#"
        // static HELLO: &str = "Hello, world!";
        // fn main() {
        //     println!("{HELLO}");
        // }
        // "#
        //     .into(),
        // },
        output_dir: None,                     // Option<PathBuf>
        output_file: None,                    // Option<PathBuf>
        file_loader: None,                    // Option<Box<dyn FileLoader + Send + Sync>>
        locale_resources: Default::default(), // rustc_driver::DEFAULT_LOCALE_RESOURCES,
        lint_caps: Default::default(),        // FxHashMap<lint::LintId, lint::Level>
        // lint_caps: FxHashMap::default(), // FxHashMap<lint::LintId, lint::Level>
        // This is a callback from the driver that is called when [`ParseSess`] is created.
        parse_sess_created: None, //Option<Box<dyn FnOnce(&mut ParseSess) + Send>>
        // This is a callback from the driver that is called when we're registering lints;
        // it is called during plugin registration when we have the LintStore in a non-shared state.
        //
        // Note that if you find a Some here you probably want to call that function in the new
        // function being registered.
        register_lints: None, // Option<Box<dyn Fn(&Session, &mut LintStore) + Send + Sync>>
        // This is a callback from the driver that is called just after we have populated
        // the list of queries.
        //
        // The second parameter is local providers and the third parameter is external providers.
        override_queries: None, // Option<fn(&Session, &mut ty::query::Providers<'_>, &mut ty::query::Providers<'_>)>
        // Registry of diagnostics codes.
        registry: registry::Registry::new(&rustc_error_codes::DIAGNOSTICS),
        make_codegen_backend: None,
    };
    rustc_interface::run_compiler(config, |compiler| {
        compiler.enter(|queries| {
            // Parse the program and print the syntax tree.
            // let parse = queries.parse().unwrap().get_mut().clone();
            // println!("{parse:#?}");

            // Analyze the program and inspect the types of definitions.
            queries.global_ctxt().unwrap().enter(|tcx| {
                for id in tcx.hir().items() {
                    // dbg!(&id);
                    let hir = tcx.hir();
                    let item = hir.item(id);
                    // dbg!(&item);
                    match item.kind {
                        rustc_hir::ItemKind::Static(_, _, _) | rustc_hir::ItemKind::Fn(_, _, _) => {
                            let name = item.ident;
                            let ty = tcx.type_of(item.hir_id().owner.def_id);
                            println!("{name:?}:\t{ty:?}")
                        }
                        _ => (),
                    }
                }
            })
        });
    });
}