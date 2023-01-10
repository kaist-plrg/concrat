#! /bin/bash

sed -i '5567s/&mut//' main.rs
sed -i '29285s/-/.wrapping_sub(/' main.rs
sed -i '29285s/);/));/' main.rs
