#! /bin/bash

sed -i '2546s/.*//g' main.rs
sed -i 's/== _ as /== 0 as /g' main.rs
sed -i 's/ = _;/ = 0 as _;/g' main.rs
sed -i 's/, _)/, 0 as _)/g' main.rs
