#! /bin/bash

sed -i '6064s/libc::c_void/_/g' main.rs
sed -i '11580s/libc::c_void/_/g' main.rs
sed -i '11813s/libc::c_void/_/g' main.rs
sed -i '12640s/libc::c_void/_/g' main.rs
sed -i '12665s/libc::c_void/_/g' main.rs
sed -i '17255s/libc::c_void/_/g' main.rs
sed -i '17476s/libc::c_void/_/g' main.rs
