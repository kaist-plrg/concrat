#! /bin/bash

sed -i '1030,1033s/.*//g' main.rs
sed -i '1041,1044s/.*//g' main.rs
sed -i '1050,1053s/.*//g' main.rs
sed -i '1059,1062s/.*//g' main.rs
sed -i '1070,1073s/.*//g' main.rs
sed -i '1077,1080s/.*//g' main.rs
sed -i '1087,1090s/.*//g' main.rs
sed -i '1095s/.*//g' main.rs
sed -i '1099s/.*//g' main.rs
sed -i '1109,1114s/.*//g' main.rs
sed -i '5759,5762s/.*//g' main.rs
sed -i '5767,5770s/.*//g' main.rs
sed -i '7281,7284s/.*//g' main.rs
sed -i '7289s/.*//g' main.rs
sed -i '7660,7663s/.*//g' main.rs
sed -i '8117s/.*//g' main.rs
sed -i 's/lua.as_ptr() as \*mut _/lua.as_ptr()/g' main.rs
