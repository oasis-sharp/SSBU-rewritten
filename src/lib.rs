#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros,
    clippy::borrow_interior_mutable_const
)]

mod ike;
mod ganon;
mod mewtwo;
mod sonic;
mod zelda;
mod mariod;
mod rockman;


#[skyline::main(name = "smashline_test")]
pub fn main() {
    ike::install();
    ganon::install();
    mewtwo::install();
    sonic::install();
    zelda::install();
    mariod::install();
    rockman::install();
}