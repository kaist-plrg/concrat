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

use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    path::PathBuf,
};

use clap::{App, Arg};
use concrat::*;
use etrace::some_or;

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
        .get_matches();
    let mut input = PathBuf::from(matches.value_of("input").unwrap());
    let dep = PathBuf::from(matches.value_of("dependency").unwrap());

    input.push("cfg.dot");
    let cfg = fs::read_to_string(&input).unwrap();
    input.pop();

    let mut node_cline: HashMap<_, HashSet<_>> = HashMap::new();
    for s in cfg.split('\n') {
        let i = some_or!(s.find(" ->"), continue);
        let node = &s[1..i];
        let j = some_or!(s.find("label = \""), continue);
        let k = some_or!(s.rfind('"'), continue);
        let line = &s[(j + 9)..k];
        let m = node_cline.entry(node.to_string()).or_default();
        for l in line.split(',') {
            if let Ok(l) = l.parse::<usize>() {
                m.insert(l);
            }
        }
    }

    input.push("lines");
    let lines = fs::read_to_string(&input).unwrap();
    input.pop();

    let mut cline_rline: HashMap<_, HashSet<_>> = HashMap::new();
    for s in lines.split('\n') {
        let i = some_or!(s.find('L'), continue);
        let j = some_or!(s.find(' '), continue);
        let cline = &s[(i + 1)..j];
        let cline: usize = some_or!(cline.parse().ok(), continue);
        let rline = &s[(j + 1)..];
        let rline: usize = some_or!(rline.parse().ok(), continue);
        cline_rline.entry(cline).or_default().insert(rline);
    }

    let empty = HashSet::new();
    let node_rline: HashMap<_, HashSet<_>> = node_cline
        .drain()
        .map(|(n, c)| {
            (
                n,
                c.iter()
                    .flat_map(|l| cline_rline.get(l).unwrap_or(&empty))
                    .cloned()
                    .collect(),
            )
        })
        .collect();

    input.push("a.xml");
    let elements = parse_xml::parse_file(input.to_str().unwrap());
    input.pop();

    input.push("c2rust-lib.rs");
    let args = util::compile_args(&input, &dep);
    input.pop();

    let code_summary = validate::collect_definitions(args);

    input.push("a.json");
    let file = File::create(input.to_str().unwrap()).unwrap();
    input.pop();

    let summary = analysis::summarize(elements, &code_summary, &node_rline);
    serde_json::to_writer_pretty(file, &summary).unwrap();
}
