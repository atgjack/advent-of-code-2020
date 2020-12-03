#![feature(min_const_generics)]

use day::Day;
use one::DayOne;
use two::DayTwo;

mod day;
mod one;
mod two;

fn main() {
    // DayOne::new(include_str!("input/one.txt")).print_solutions(1);
    DayTwo::new(include_str!("input/two.txt")).print_solutions(2);
}
