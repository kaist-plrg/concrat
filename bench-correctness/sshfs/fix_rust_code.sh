#! /bin/bash

sed -i '5iprintln!("cargo:rustc-link-arg=-lfuse3");' build.rs
sed -i '5iprintln!("cargo:rustc-link-arg=-lglib-2.0");' build.rs
