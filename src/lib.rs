#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_driver_impl;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_session;
extern crate rustc_span;

mod config;

use std::path::PathBuf;

use rustc_hir::ItemKind;

pub struct Megadep {
    pub deps_dir: String,
}

impl Megadep {
    pub fn process(&self, crate_name: &str, path: PathBuf) {
        holochain_trace::test_run().ok();

        let deps_dir = &self.deps_dir;

        let opts = config::parse_opts(&format!(
            "--crate-name {crate_name} --edition 2021 --crate-type lib -L {deps_dir} --extern alpha",
            // "--edition 2021 --crate-type lib -L {deps_dir} --extern alpha --sysroot {sysroot}",
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

                    let money = |def_id| -> String {
                        // tcx.def_path_debug_str(def_id)
                        tcx.def_path_str(def_id)
                    };

                    for id in hir.items() {
                        // dbg!(&id);
                        let hir_id = id.hir_id();
                        let item = hir.item(id);
                        let node = hir.get(hir_id);

                        let name = item.ident;
                        match item.kind {
                            ItemKind::Static(_, _, _)
                            | ItemKind::Trait(_, _, _, _, _)
                            | ItemKind::TraitAlias(_, _)
                            | ItemKind::TyAlias(_, _)
                            | ItemKind::Fn(_, _, _)
                            | ItemKind::Struct(_, _)
                            | ItemKind::Enum(_, _) => {
                                println!("NODE> {} {:?}, {:#?}", name, node.ident(), node);
                            }
                            _ => (),
                        }
                        // dbg!(&item.kind, &item.ident);
                        match item.kind {
                            ItemKind::Static(_, _, _)
                            | ItemKind::Trait(_, _, _, _, _)
                            | ItemKind::TraitAlias(_, _)
                            | ItemKind::TyAlias(_, _)
                            | ItemKind::Enum(_, _) => {
                                // | ItemKind::Fn(_, _, _){
                                let m = money(item.owner_id.to_def_id());

                                let ty1 = tcx.type_of(item.owner_id.def_id).subst_identity();
                                let ty2 = tcx.type_of(item.hir_id().owner.def_id).subst_identity();
                                println!("{m:#?}  {name:?}   {ty1:?}   {ty2:?}")
                            }

                            ItemKind::Struct(v, _) => match v {
                                rustc_hir::VariantData::Struct(_, _) => todo!(),
                                rustc_hir::VariantData::Tuple(fs, hid, lid) => {
                                    let fs: Vec<_> = fs
                                        .into_iter()
                                        .map(|f| money(f.def_id.to_def_id()))
                                        .collect();
                                    println!("Struct/Tuple     {name}({fs:?})");
                                }
                                rustc_hir::VariantData::Unit(hid, lid) => {
                                    println!("Struct/Unit      {name}:   {hid:?}");
                                }
                            },
                            ItemKind::Fn(sig, _, body) => {
                                let ins = sig
                                    .decl
                                    .inputs
                                    .iter()
                                    .map(|t| money(t.hir_id.owner.to_def_id()))
                                    .collect::<Vec<_>>();
                                let out = match sig.decl.output {
                                    rustc_hir::FnRetTy::DefaultReturn(_) => continue,
                                    rustc_hir::FnRetTy::Return(r) => {
                                        money(r.hir_id.owner.to_def_id())
                                    }
                                };

                                let ty = tcx.type_of(item.hir_id().owner.def_id).subst_identity();
                                let bod = tcx.type_of(body.hir_id.owner.def_id).skip_binder();
                                println!("{name:?}:\t{ty:?}\t{bod:?}");
                                println!("{name:?}:      {ins:?}      ->      {out:?}");
                            }
                            _ => (),
                        }
                    }
                })
            });
        });
    }
}

macro_rules! expect_v {
    ($e:expr, $p:path) => {
        match $e {
            $p(value) => value,
            _ => panic!("expected {}", stringify!($p)),
        }
    };
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
