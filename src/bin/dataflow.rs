#![deny(absolute_paths_not_starting_with_crate)]
#![deny(deprecated_in_future)]
#![deny(elided_lifetimes_in_paths)]
#![deny(explicit_outlives_requirements)]
#![deny(keyword_idents)]
#![deny(macro_use_extern_crate)]
#![deny(meta_variable_misuse)]
#![deny(missing_abi)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![deny(non_ascii_idents)]
#![deny(noop_method_call)]
#![deny(pointer_structural_match)]
#![deny(rust_2021_incompatible_closure_captures)]
#![deny(rust_2021_incompatible_or_patterns)]
#![deny(rust_2021_prefixes_incompatible_syntax)]
#![deny(rust_2021_prelude_collisions)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unreachable_pub)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_extern_crates)]
#![deny(unused_import_braces)]
#![deny(unused_lifetimes)]
#![deny(unused_qualifications)]
#![deny(warnings)]

use std::{fs::File, path::PathBuf};

use clap::{App, Arg};
use concrat::*;

fn main() {
    let matches = App::new("Analysis")
        .arg(
            Arg::with_name("input")
                .long("input")
                .short("i")
                .help("input directory")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dependency")
                .long("dependency")
                .short("d")
                .help("dependency directory")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .help("verbose")
                .takes_value(false),
        )
        .get_matches();
    let verbose = matches.is_present("verbose");
    let mut input = PathBuf::from(matches.value_of("input").unwrap());
    let dep = PathBuf::from(matches.value_of("dependency").unwrap());

    input.push("c2rust-lib.rs");
    let args = util::compile_args(&input, &dep);
    input.pop();

    input.push("a.json");
    let file = File::create(input.to_str().unwrap()).unwrap();
    input.pop();

    let summary = dataflow::run(args, verbose);
    serde_json::to_writer_pretty(file, &summary).unwrap();
}
