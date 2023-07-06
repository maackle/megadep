use std::path::PathBuf;

use rustc_session::config::CheckCfg;
use rustc_session::config::Options;
use rustc_session::EarlyErrorHandler;

pub fn config(path: PathBuf, opts: Options) -> rustc_interface::Config {
    rustc_interface::Config {
        // Command line options
        opts,
        // cfg! configuration in addition to the default ones
        crate_cfg: Default::default(), // FxHashSet<(String, Option<String>)>
        crate_check_cfg: CheckCfg::default(), // CheckCfg
        input: rustc_session::config::Input::File(path),
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
        registry: rustc_errors::registry::Registry::new(&rustc_error_codes::DIAGNOSTICS),
        make_codegen_backend: None,
    }
}

pub fn parse_opts(args: impl ToString) -> Options {
    let args = args.to_string();
    let args: Vec<String> = args
        .split(" ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    // let matches = rustc_session::getopts::Options::default()
    //     .parse(args)
    //     .unwrap();
    let matches = rustc_driver_impl::handle_options(&mut eeh(), args.as_slice()).unwrap();
    rustc_session::config::build_session_options(&mut eeh(), &matches)
}

fn eeh() -> EarlyErrorHandler {
    EarlyErrorHandler::new(Default::default())
}
