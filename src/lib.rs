#![feature(drain_filter)]
#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_span;

pub mod parse_xml;
pub mod rewrite;
