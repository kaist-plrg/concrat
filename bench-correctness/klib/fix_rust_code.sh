#! /bin/bash

sed -i '364s/libc::c_void/_/g' main.rs
sed -i '365s/libc::c_void/_/g' main.rs
sed -i '751s/libc::c_void/_/g' main.rs
sed -i '754s/libc::c_void/_/g' main.rs
