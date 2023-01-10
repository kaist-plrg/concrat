#! /bin/bash

sed -i '12i[build-dependencies]' Cargo.toml
sed -i '13icc = "1.0"' Cargo.toml
sed -i '5icc::Build::new().file("acosw.S").compile("acosw");' build.rs
sed -i '234i#[no_mangle]' main.rs
