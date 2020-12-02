#![feature(min_const_generics)]

use day::Day;
use one::DayOne;

mod day;
mod one;

fn main() {
    DayOne::new(include_str!("input/one.txt")).print_solutions(1);
}
