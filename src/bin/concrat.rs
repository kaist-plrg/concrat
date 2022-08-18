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

use std::{fs::File, io::Write, path::PathBuf, time::Instant};

use clap::{App, Arg};
use concrat::*;

fn main() {
    let start = Instant::now();

    let matches = App::new("Concrat")
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
            Arg::with_name("dry-run")
                .long("dry-run")
                .help("do not fix files")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .takes_value(false),
        )
        .get_matches();
    let mut input = PathBuf::from(matches.value_of("input").unwrap());
    let dep = PathBuf::from(matches.value_of("dependency").unwrap());
    let dry_run = matches.is_present("dry-run");
    let verbose = matches.is_present("verbose");

    input.push("a.json");
    let summary = analysis::AnalysisSummary::from_json_file(&input);
    input.pop();

    if verbose {
        summary.pretty_print();
    }

    input.push("c2rust-lib.rs");
    let args = util::compile_args(&input, &dep);
    input.pop();

    let replacements = rewrite::collect_replacements(args, summary);

    if verbose {
        for replacement in &replacements {
            println!("- replace `{}`", replacement.snippet.text.1);
            println!("  with    `{}`", replacement.replacement);
            println!(
                "  at {}:{}-{}:{}",
                replacement.snippet.line_range.start.line,
                replacement.snippet.line_range.start.column,
                replacement.snippet.line_range.end.line,
                replacement.snippet.line_range.end.column,
            );
        }
    }

    let fixed_source_code = rewrite::apply_suggestions(replacements);
    if dry_run {
        println!("{}", fixed_source_code);
    } else {
        input.push("main.rs");
        let mut file = File::create(input.to_str().unwrap()).unwrap();
        file.write_all(fixed_source_code.as_bytes()).unwrap();
    }

    println!("{:?}", start.elapsed());
}
