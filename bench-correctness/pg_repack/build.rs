#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
println!("cargo:rustc-link-arg=-L/usr/lib/postgresql/13/lib");
println!("cargo:rustc-link-arg=-lpq");
println!("cargo:rustc-link-arg=-lpgcommon");
println!("cargo:rustc-link-arg=-lpgport");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
