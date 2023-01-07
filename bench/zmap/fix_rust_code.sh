#! /bin/bash

sed -i 's/) as libc::size_t/) as _/g' main.rs
sed -i '4784s/.*//g' main.rs
sed -i '5024s/.*//g' main.rs
sed -i '4915s/.*//g' main.rs
sed -i '8499s/.*//g' main.rs
sed -i '8858s/.*//g' main.rs
sed -i '18263,18270s/.*//g' main.rs
sed -i '22376,22382s/.*//g' main.rs
sed -i '22400,22406s/.*//g' main.rs
sed -i '22424,22430s/.*//g' main.rs
sed -i '23442,23445s/.*//g' main.rs
sed -i '23765,23767s/.*//g' main.rs
sed -i '23768s/.*/0/g' main.rs
