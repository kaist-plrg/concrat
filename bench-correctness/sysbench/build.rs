#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
println!("cargo:rustc-link-arg=-lmysqlclient");
println!("cargo:rustc-link-arg=-laio");
println!("cargo:rustc-link-lib=static=sbmysql");
println!("cargo:rustc-link-lib=static=sbthreads");
println!("cargo:rustc-link-lib=static=sbmutex");
println!("cargo:rustc-link-lib=static=sbmemory");
println!("cargo:rustc-link-lib=static=sbfileio");
println!("cargo:rustc-link-lib=static=sbcpu");
println!("cargo:rustc-link-lib=static=luajit-5.1");
println!("cargo:rustc-link-search=native=.");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
