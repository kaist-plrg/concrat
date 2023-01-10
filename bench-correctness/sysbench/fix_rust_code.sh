#! /bin/bash

sed -i '1043,1046s/.*//g' main.rs
sed -i '1052,1055s/.*//g' main.rs
sed -i '1060,1063s/.*//g' main.rs
sed -i '1067,1070s/.*//g' main.rs
sed -i '2775,2778s/.*//g' main.rs
sed -i '2783s/.*//g' main.rs
sed -i '37592,37595s/.*//g' main.rs
sed -i '38053s/.*//g' main.rs
sed -i '38187,38190s/.*//g' main.rs
sed -i '38198,38201s/.*//g' main.rs
sed -i '38209,38212s/.*//g' main.rs
sed -i '38381,38384s/.*//g' main.rs
sed -i '38392,38395s/.*//g' main.rs
sed -i '38400s/.*//g' main.rs
sed -i '38404s/.*//g' main.rs
sed -i '38414,38419s/.*//g' main.rs
sed -i 's/lua.as_ptr() as \*mut _/lua.as_ptr()/g' main.rs

sed -i '1115i#[no_mangle]' main.rs
sed -i '2061i#[no_mangle]' main.rs
sed -i '2703i#[no_mangle]' main.rs
sed -i '3498i#[no_mangle]' main.rs
sed -i '3552i#[no_mangle]' main.rs
sed -i '3615i#[no_mangle]' main.rs
sed -i '36742i#[no_mangle]' main.rs
sed -i '36809i#[no_mangle]' main.rs
sed -i '36877i#[no_mangle]' main.rs
sed -i '36958i#[no_mangle]' main.rs
sed -i '36991i#[no_mangle]' main.rs
sed -i '37018i#[no_mangle]' main.rs
sed -i '37050i#[no_mangle]' main.rs
sed -i '37063i#[no_mangle]' main.rs
sed -i '37600i#[no_mangle]' main.rs
sed -i '37823i#[no_mangle]' main.rs
sed -i '37830i#[no_mangle]' main.rs
sed -i '38262i#[no_mangle]' main.rs
sed -i '38377i#[no_mangle]' main.rs
sed -i '38392i#[no_mangle]' main.rs
sed -i '38495i#[no_mangle]' main.rs
sed -i '39139i#[no_mangle]' main.rs

sed -i '5iprintln!("cargo:rustc-link-search=native=.");' build.rs
sed -i '5iprintln!("cargo:rustc-link-lib=static=luajit-5.1");' build.rs
sed -i '5iprintln!("cargo:rustc-link-lib=static=sbcpu");' build.rs
sed -i '5iprintln!("cargo:rustc-link-lib=static=sbfileio");' build.rs
sed -i '5iprintln!("cargo:rustc-link-lib=static=sbmemory");' build.rs
sed -i '5iprintln!("cargo:rustc-link-lib=static=sbmutex");' build.rs
sed -i '5iprintln!("cargo:rustc-link-lib=static=sbthreads");' build.rs
sed -i '5iprintln!("cargo:rustc-link-lib=static=sbmysql");' build.rs
sed -i '5iprintln!("cargo:rustc-link-arg=-laio");' build.rs
sed -i '5iprintln!("cargo:rustc-link-arg=-lmysqlclient");' build.rs
