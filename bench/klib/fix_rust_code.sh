#! /bin/bash

sed -i '361s/libc::c_void/_/g' main.rs
sed -i '362s/libc::c_void/_/g' main.rs
sed -i '748s/libc::c_void/_/g' main.rs
sed -i '751s/libc::c_void/_/g' main.rs
