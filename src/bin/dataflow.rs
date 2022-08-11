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

use std::{collections::HashSet, fs::File, path::PathBuf};

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
        .arg(
            Arg::with_name("test")
                .long("test")
                .short("t")
                .help("test")
                .takes_value(false),
        )
        .get_matches();
    let verbose = matches.is_present("verbose");
    let test = matches.is_present("test");
    let mut input = PathBuf::from(matches.value_of("input").unwrap());
    let dep = PathBuf::from(matches.value_of("dependency").unwrap());

    input.push("c2rust-lib.rs");
    let args = util::compile_args(&input, &dep);
    input.pop();

    let summary = dataflow::run(args, verbose);

    if test {
        input.push("b.json");
        let file = File::open(input.to_str().unwrap()).unwrap();
        input.pop();

        let summary2: analysis::AnalysisSummary = serde_json::from_reader(file).unwrap();
        assert_eq!(summary.mutex_map, summary2.mutex_map, "mutex_map");
        assert_eq!(
            summary.array_mutex_map, summary2.array_mutex_map,
            "array_mutex_map"
        );
        assert_eq!(
            summary.struct_mutex_map, summary2.struct_mutex_map,
            "struct_mutex_map"
        );
        assert_eq!(
            summary.function_map.keys().collect::<HashSet<_>>(),
            summary2.function_map.keys().collect::<HashSet<_>>()
        );
        for f in summary.function_map.keys() {
            assert_eq!(
                summary.function_map.get(f).unwrap(),
                summary2.function_map.get(f).unwrap(),
                "{}",
                f
            );
        }
    } else {
        input.push("a.json");
        let file = File::create(input.to_str().unwrap()).unwrap();
        input.pop();

        serde_json::to_writer_pretty(file, &summary).unwrap();
    }
}
