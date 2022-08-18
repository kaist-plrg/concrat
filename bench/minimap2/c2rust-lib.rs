#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(register_tool)]
#![feature(rustc_private)]
#![feature(stdsimd)]
#![feature(untagged_unions)]
#![register_tool(c2rust)]

#[macro_use]
extern crate c2rust_bitfields;#[macro_use]
extern crate c2rust_asm_casts;
extern crate libc;
pub mod main;
