#! /bin/bash

sed -i '4638s/&mut //g' main.rs
sed -i '5580s/libc::c_void/_/g' main.rs
