#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_driver_impl;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

mod config;

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use rustc_hir::{def_id::DefId, intravisit, HirId};
use rustc_middle::ty::TyCtxt;

pub struct Megadep {
    pub deps_dir: String,
}

pub type DefGraphMap<T = DefId> = HashMap<T, HashSet<T>>;

#[derive(Default, Debug)]
pub struct DefGraph<T = DefId>(DefGraphMap<T>);

impl<T: Eq + PartialEq + std::hash::Hash> DefGraph<T> {
    pub fn extend(mut self, other: Self) -> Self {
        for (k, v) in other.0 {
            self.0.entry(k).or_default().extend(v);
        }
        self
    }
}

struct Vis<'v> {
    cx: TyCtxt<'v>,
    graph: DefGraphMap<String>,
    paths: Vec<DefId>,
    // paths: Vec<rustc_hir::Path<'v>>,
}

impl<'v> Vis<'v> {
    fn node(&self, id: HirId) -> rustc_hir::Node {
        self.cx.hir().get(id)
    }

    fn repr(&self, did: DefId) -> String {
        self.cx.def_path_debug_str(did)
    }
}

impl<'v> intravisit::Visitor<'v> for Vis<'v> {
    // fn visit_name(&mut self, name: rustc_span::Symbol) {
    //     // dbg!(name);
    // }

    // fn visit_id(&mut self, id: HirId) {
    //     let node = self.node(id);
    //     // dbg!(&node);

    //     let owner = id.owner.to_def_id();
    //     let parent = self.cx.parent(owner);
    //     // let parent = self.cx.hir().parent_id(id);
    //     // self.cx.item_name(id);
    //     if parent != owner {}
    // }

    fn visit_path(&mut self, path: &rustc_hir::Path<'v>, id: HirId) {
        dbg!(path.res);
        let hir = self.cx.hir();
        match path.res {
            rustc_hir::def::Res::Def(_, did) => {
                // dbg!(&path.res);
                dbg!(self.cx.def_path_debug_str(path.res.def_id()));
                self.paths.push(did);
                let didstr = self.repr(did);
                if let Some((parent_id, _)) = hir.parent_owner_iter(id).next() {
                    let m = self.graph.entry(self.repr(parent_id.to_def_id())).or_default();
                    m.insert(didstr);
                } else {
                    dbg!("no parent");
                }
            }
            _ => ()
            // rustc_hir::def::Res::PrimTy(_) => todo!(),
            // rustc_hir::def::Res::SelfTyParam { trait_ } => todo!(),
            // rustc_hir::def::Res::SelfTyAlias { alias_to, forbid_generic, is_trait_impl } => todo!(),
            // rustc_hir::def::Res::SelfCtor(_) => todo!(),
            // rustc_hir::def::Res::Local(_) => todo!(),
            // rustc_hir::def::Res::ToolMod => todo!(),
            // rustc_hir::def::Res::NonMacroAttr(_) => todo!(),
            // rustc_hir::def::Res::Err => todo!(),
        }
        intravisit::walk_path(self, path)
    }
    // fn visit_item(&mut self, i: &'v rustc_hir::Item<'v>) {
    //     dbg!(i);
    //     intravisit::walk_item(self, i)
    // }
}

// type ProcessArgs<'a> = Vec<(&'a str, Vec<PathBuf>)>;

impl Megadep {
    pub fn process(&self, crate_name: &str, path: PathBuf) -> DefGraph<String> {
        holochain_trace::test_run().ok();

        let deps_dir = &self.deps_dir;

        let opts = config::parse_opts(&format!(
            "--crate-name {crate_name} --edition 2021 --crate-type lib -L {deps_dir} --extern alpha",
            // "--edition 2021 --crate-type lib -L {deps_dir} --extern alpha --sysroot {sysroot}",
        ));

        let config = config::config(path, opts);

        let mut graph = HashMap::new();

        rustc_interface::run_compiler(config, |compiler| {
            compiler.enter(|queries| {
                // Parse the program and print the syntax tree.
                // let parse = queries.parse().unwrap().get_mut().clone();
                // println!("{parse:#?}");

                // Analyze the program and inspect the types of definitions.
                queries.global_ctxt().unwrap().enter(|tcx| {
                    // let r = tcx.typeck(todo!());
                    let hir = tcx.hir();
                    // dbg!(tcx.hir_crate_items(()));

                    let mut v = Vis {
                        cx: tcx,
                        graph: Default::default(),
                        paths: Default::default(),
                    };
                    hir.visit_all_item_likes_in_crate(&mut v);
                    dbg!(&v.paths, &v.graph);
                    graph = v.graph;
                })
            })
        });

        return DefGraph(graph);
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
