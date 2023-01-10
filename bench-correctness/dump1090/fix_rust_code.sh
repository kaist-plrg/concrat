#! /bin/bash

sed -i '5iprintln!("cargo:rustc-link-arg=-lrtlsdr");' build.rs
