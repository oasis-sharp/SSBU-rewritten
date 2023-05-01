#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(warnings, unused)]

mod inkling;
mod gekkouga;
mod wolf;
mod common;
mod ike;


#[skyline::main(name = "smashline_test")]
pub fn main() {
    inkling::install();
    gekkouga::install();
    wolf::install();
    ike::install();
    common::install();
}