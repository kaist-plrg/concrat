#! /bin/bash

sed -i 's/) as libc::size_t/) as _/g' main.rs

sed -i '5iprintln!("cargo:rustc-link-arg=-lpgport");' build.rs
sed -i '5iprintln!("cargo:rustc-link-arg=-lpgcommon");' build.rs
sed -i '5iprintln!("cargo:rustc-link-arg=-lpq");' build.rs
sed -i '5iprintln!("cargo:rustc-link-arg=-L/usr/lib/postgresql/13/lib");' build.rs
