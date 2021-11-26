use concrat::*;

fn main() {
    let (funcs, calls, globs) = parse_xml::parse_file("/home/medowhill/out.xml");
    let mutex_map = parse_xml::generate_mutex_map(&globs);
    let node_map = parse_xml::generate_node_map(&calls);
    let function_map = parse_xml::generate_function_map(&funcs, &node_map);

    let args: Vec<_> = vec![
        "create-initial-program",
        "/home/medowhill/simple/c2rust-lib.rs",
        "--sysroot",
        "/home/medowhill/.rustup/toolchains/nightly-2021-11-24-x86_64-unknown-linux-gnu",
        "--crate-type",
        "lib",
        "-A",
        "warnings",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let suggestions = rewrite::collect_suggestions(args, mutex_map, function_map);

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

    for (file, suggestions) in suggestions {
        let fixed_source_code = rewrite::apply_suggestions(file, suggestions);
        use std::fs::File;
        use std::io::Write;
        let mut file = File::create("/home/medowhill/all/all.rs").unwrap();
        file.write_all(fixed_source_code.as_bytes()).unwrap();
    }
}
