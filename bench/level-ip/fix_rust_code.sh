#! /bin/bash

sed -i '1i#![allow(unaligned_references)]' c2rust-lib.rs
sed -i '4442s/libc::c_void/_/g' main.rs
sed -i '4494s/libc::c_void/_/g' main.rs
sed -i '4691s/libc::c_void/_/g' main.rs
sed -i '4819s/libc::c_void/_/g' main.rs
sed -i '4896s/libc::c_void/_/g' main.rs
sed -i '4963s/libc::c_void/_/g' main.rs
