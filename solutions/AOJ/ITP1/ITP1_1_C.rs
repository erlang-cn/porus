#![feature(proc_macro_non_items)]
#![cfg_attr(not(debug_assertions), no_std)]

#[macro_use]
extern crate porus;
prelude!();

fn solve() {
    let a: isize = read!();
    let b: isize = read!();
    writelnf!("{:d} {:d}", a * b, (a + b) * 2);
}
