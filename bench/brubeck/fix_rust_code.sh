#! /bin/bash

sed -i '3186s/libc::c_void/_/g' main.rs
sed -i '3566s/libc::c_void/_/g' main.rs
