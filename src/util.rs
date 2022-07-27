use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    path::{Path, PathBuf},
    process::Command,
};

pub fn compile_args(input: &Path, dep: &Path) -> Vec<String> {
    let dep = dep.to_str().unwrap();
    let dependency = format!("dependency={}", dep);
    let asm_casts = format!(
        "c2rust_asm_casts={}/libc2rust_asm_casts-5452fb8e557bf4f0.rlib",
        dep
    );
    let bitfields = format!(
        "c2rust_bitfields={}/libc2rust_bitfields-dea51de3183f0659.rlib",
        dep
    );
    vec![
        "create-initial-program",
        input.to_str().unwrap(),
        "--sysroot",
        sys_root().as_str(),
        "--crate-type",
        "lib",
        "-A",
        "warnings",
        "-L",
        &dependency,
        "--extern",
        &asm_casts,
        "--extern",
        &bitfields,
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
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

pub fn transitive_closure<T: Clone + Eq + Hash>(
    mut map: HashMap<T, HashSet<T>>,
) -> HashMap<T, HashSet<T>> {
    let empty = HashSet::new();
    loop {
        let new = map
            .iter()
            .map(|(k, vs)| {
                let nvs = vs
                    .iter()
                    .flat_map(|v| map.get(v).unwrap_or(&empty).clone())
                    .collect();
                (k.clone(), vs.union(&nvs).cloned().collect())
            })
            .collect();
        if map == new {
            return new;
        }
        map = new;
    }
}
