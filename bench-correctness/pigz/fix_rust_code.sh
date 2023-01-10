#! /bin/bash

sed -i '12075s/) {/) -> ! {/' main.rs

sed -i '5iprintln!("cargo:rustc-link-arg=-lz");' build.rs
