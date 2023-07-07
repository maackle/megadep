#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_driver_impl;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_session;
extern crate rustc_span;

mod config;

use std::{collections::BTreeMap, path::PathBuf, process, str};

use rustc_session::config::{default_lib_output, CrateType, ExternEntry, ExternLocation, Options};
use rustc_session::search_paths::SearchPath;
use rustc_session::EarlyErrorHandler;
use rustc_span::edition::Edition;

// use rustc_hash::{FxHashMap, FxHashSet};
// use rustc_span::source_map;

pub struct Megadep {
    pub deps_dir: String,
}

impl Megadep {
    pub fn process(&self, path: PathBuf) {
        holochain_trace::test_run().ok();

        let opts = config::parse_opts(&format!(
            "--edition 2021 --crate-type lib -L {} --extern alpha",
            self.deps_dir
        ));

        let config = config::config(path, opts);

        rustc_interface::run_compiler(config, |compiler| {
            compiler.enter(|queries| {
                // Parse the program and print the syntax tree.
                // let parse = queries.parse().unwrap().get_mut().clone();
                // println!("{parse:#?}");

                // Analyze the program and inspect the types of definitions.
                queries.global_ctxt().unwrap().enter(|tcx| {
                    let hir = tcx.hir();
                    for id in hir.items() {
                        // dbg!(&id);
                        let item = hir.item(id);
                        dbg!(&item.kind, &item.ident);
                        match item.kind {
                            rustc_hir::ItemKind::Static(_, _, _)
                            | rustc_hir::ItemKind::Fn(_, _, _) => {
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
}
/*

   let out = process::Command::new("rustc")
       .arg("--print=sysroot")
       .current_dir(".")
       .output()
       .unwrap();
   let sysroot = str::from_utf8(&out.stdout).unwrap().trim();
   let mut externs = BTreeMap::new();
   let location = ExternLocation::FoundInLibrarySearchDirectories;
   externs.insert(
       "alpha".to_string(),
       ExternEntry {
           location: ExternLocation::FoundInLibrarySearchDirectories,
           is_private_dep: false,
           add_prelude: false,
           nounused_dep: false,
           force: false,
       },
   );
   // externs.insert(
   //     "beta".to_string(),
   //     ExternEntry {
   //         location,
   //         is_private_dep: false,
   //         add_prelude: false,
   //         nounused_dep: false,
   //         force: false,
   //     },
   // );

   let search_paths = vec![SearchPath::from_cli_opt(
       &EarlyErrorHandler::new(Default::default()),
       &deps_dir.to_str().expect("bad deps dir"),
   )];

   dbg!(&search_paths[0].files.len());
   dbg!(&externs);

   let opts0 = Options {
       // maybe_sysroot: Some(path::PathBuf::from(sysroot)),
       search_paths,
       externs: rustc_session::config::Externs::new(externs),
       edition: Edition::Edition2021,
       crate_types: vec![default_lib_output()],
       ..Options::default()
   };

*/
