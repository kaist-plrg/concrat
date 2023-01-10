#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
cc::Build::new().file("./ksw2_ll_sse.c").file("./ksw2_extd2_sse.c").file("./ksw2_exts2_sse.c").file("./ksw2_extz2_sse.c").compile("ksw2");
println!("cargo:rustc-link-arg=-lz");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
