#! /bin/bash

sed -i '5iprintln!("cargo:rustc-link-search=native=.");' build.rs
sed -i '5iprintln!("cargo:rustc-link-lib=static=snoopy-test-cli");' build.rs
