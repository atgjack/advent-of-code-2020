use std::fmt::Display;

pub trait Day<A: Display, B: Display> {
    fn part_a(&self) -> A;
    fn part_b(&self) -> B;

    fn print_solutions(&self, num: usize) {
        println!("{}.a = \t{}", num, self.part_a());
        println!("{}.b = \t{}", num, self.part_b());
    } 
}
