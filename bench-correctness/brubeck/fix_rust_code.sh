#! /bin/bash

sed -i 's/ck_ht_entry_t {/ck_ht_entry(ck_ht_entry_Inner {/g' main.rs
sed -i '2477s/}/})/g' main.rs
sed -i '2501s/}/})/g' main.rs
sed -i '5553s/}/})/g' main.rs
sed -i '5583s/}/})/g' main.rs
sed -i '3091s/libc::c_void/_/g' main.rs
sed -i '3471s/libc::c_void/_/g' main.rs

sed -i '5iprintln!("cargo:rustc-link-lib=dylib=ck");' build.rs
sed -i '6iprintln!("cargo:rustc-link-search=native=.");' build.rs
