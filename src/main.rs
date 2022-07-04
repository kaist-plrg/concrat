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

use std::{fs::File, io::Write, path::PathBuf, process::Command};

use clap::{App, Arg};
use concrat::*;

fn main() {
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
    let dry_run = matches.is_present("dry-run");
    let verbose = matches.is_present("verbose");

    input.push("a.xml");
    let elements = parse_xml::parse_file(input.to_str().unwrap());
    input.pop();

    let summary = analysis::summarize(elements);

    if verbose {
        summary.pretty_print();
    }

    input.push("c2rust-lib.rs");
    let args: Vec<_> = vec![
        "create-initial-program",
        input.to_str().unwrap(),
        "--sysroot",
        sys_root().as_str(),
        "--crate-type",
        "lib",
        "-A",
        "warnings",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    input.pop();

    let suggestions = rewrite::collect_suggestions(args, summary);

    if verbose {
        for (file, suggestions) in suggestions.iter() {
            println!("For file {}:", file.to_str().unwrap());

            for suggestion in suggestions.values().flatten() {
                let solution = &suggestion.solutions[0];
                println!("{}", solution.message);
                for replacement in &solution.replacements {
                    println!(" - replace {:?}", replacement.snippet.text);
                    println!("   with   `{}`", replacement.replacement);
                    println!(
                        "   at {} {}:{}-{}:{}",
                        replacement.snippet.file_name,
                        replacement.snippet.line_range.start.line,
                        replacement.snippet.line_range.start.column,
                        replacement.snippet.line_range.end.line,
                        replacement.snippet.line_range.end.column,
                    );
                }
            }
        }
    }

    for (file, suggestions) in suggestions {
        let fixed_source_code = rewrite::apply_suggestions(file, suggestions);
        if dry_run {
            println!("{}", fixed_source_code);
        } else {
            input.push("main.rs");
            let mut file = File::create(input.to_str().unwrap()).unwrap();
            file.write_all(fixed_source_code.as_bytes()).unwrap();
        }
    }
}

fn toolchain_path(home: Option<String>, toolchain: Option<String>) -> Option<PathBuf> {
    home.and_then(|home| {
        toolchain.map(|toolchain| {
            let mut path = PathBuf::from(home);
            path.push("toolchains");
            path.push(toolchain);
            path
        })
    })
}

fn sys_root() -> String {
    std::env::var("SYSROOT")
        .ok()
        .map(PathBuf::from)
        .or_else(|| {
            let home = std::env::var("RUSTUP_HOME")
                .or_else(|_| std::env::var("MULTIRUST_HOME"))
                .ok();
            let toolchain = std::env::var("RUSTUP_TOOLCHAIN")
                .or_else(|_| std::env::var("MULTIRUST_TOOLCHAIN"))
                .ok();
            toolchain_path(home, toolchain)
        })
        .or_else(|| {
            Command::new("rustc")
                .arg("--print")
                .arg("sysroot")
                .output()
                .ok()
                .and_then(|out| String::from_utf8(out.stdout).ok())
                .map(|s| PathBuf::from(s.trim()))
        })
        .or_else(|| option_env!("SYSROOT").map(PathBuf::from))
        .or_else(|| {
            let home = option_env!("RUSTUP_HOME")
                .or(option_env!("MULTIRUST_HOME"))
                .map(ToString::to_string);
            let toolchain = option_env!("RUSTUP_TOOLCHAIN")
                .or(option_env!("MULTIRUST_TOOLCHAIN"))
                .map(ToString::to_string);
            toolchain_path(home, toolchain)
        })
        .map(|pb| pb.to_string_lossy().to_string())
        .unwrap()
}
