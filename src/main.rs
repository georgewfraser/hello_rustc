#![feature(rustc_private)]

extern crate getopts;
extern crate rustc;
extern crate rustc_ast;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_interface;
extern crate rustc_span;

use self::rustc::session;
use self::rustc::session::config;
use self::rustc_ast::ast;
use self::rustc_interface::interface;
use rustc_errors::registry;
use rustc_hash::{FxHashMap, FxHashSet};
use rustc_span::source_map;

fn main() {
    let filename = "main.rs";
    let contents = "fn main() { println!(\"hello, world!\"); }";
    let errors = registry::Registry::new(&rustc_error_codes::DIAGNOSTICS);
    let config = interface::Config {
        // Command line options
        opts: config::Options {
            ..config::Options::default()
        },

        // cfg! configuration in addition to the default ones
        // FxHashSet<(String, Option<String>)>
        crate_cfg: FxHashSet::default(),

        input: config::Input::Str {
            name: source_map::FileName::Custom(String::from(filename)),
            input: String::from(contents),
        },
        // Option<PathBuf>
        input_path: None,
        // Option<PathBuf>
        output_dir: None,
        // Option<PathBuf>
        output_file: None,
        // Option<Box<dyn FileLoader + Send + Sync>>
        file_loader: None,
        diagnostic_output: session::DiagnosticOutput::Default,

        // Set to capture stderr output during compiler execution
        // Option<Arc<Mutex<Vec<u8>>>>
        stderr: None,

        // Option<String>
        crate_name: None,
        // FxHashMap<lint::LintId, lint::Level>
        lint_caps: FxHashMap::default(),

        // This is a callback from the driver that is called when we're registering lints;
        // it is called during plugin registration when we have the LintStore in a non-shared state.
        //
        // Note that if you find a Some here you probably want to call that function in the new
        // function being registered.
        // Option<Box<dyn Fn(&Session, &mut LintStore) + Send + Sync>>
        register_lints: None,

        // This is a callback from the driver that is called just after we have populated
        // the list of queries.
        //
        // The second parameter is local providers and the third parameter is external providers.
        // Option<fn(&Session, &mut ty::query::Providers<'_>, &mut ty::query::Providers<'_>)>
        override_queries: None,

        // Registry of diagnostics codes.
        registry: errors,
    };
    interface::run_compiler(config, |compiler| {
        compiler.enter(|queries| {
            let parse = queries.parse().unwrap().take();
            println!("{:?}", parse);
            let (hir, _) = queries.lower_to_hir().unwrap().take();
            println!("{:?}", hir);
        });
    });
}
