#! /bin/bash

sed -i '12272s/libc::c_void/_/g' main.rs
sed -i '12412s/libc::c_void/_/g' main.rs
