#! /bin/bash

sed -i '12i[build-dependencies]' Cargo.toml
sed -i '13icc = "1.0"' Cargo.toml
sed -i '5icc::Build::new().file("./ksw2_ll_sse.c").file("./ksw2_extd2_sse.c").file("./ksw2_exts2_sse.c").file("./ksw2_extz2_sse.c").compile("ksw2");' build.rs
sed -i '6iprintln!("cargo:rustc-link-arg=-lz");' build.rs
