#! /bin/bash

sed -i '5iprintln!("cargo:rustc-link-arg=-lpcre");' build.rs
sed -i '5iprintln!("cargo:rustc-link-arg=-llzma");' build.rs
sed -i '5iprintln!("cargo:rustc-link-arg=-lz");' build.rs
