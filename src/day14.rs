use std::{collections::HashMap, str::FromStr};
use nom::{IResult, bytes::complete::{tag, take_until}, character::{complete::{digit1}}, combinator::map_res, sequence::tuple};

#[derive(Debug)]
struct Mask { ones: u64, zeros: u64 }

#[derive(Debug)]
pub enum Value {
    Mask { ones: u64, zeros: u64, floating: Vec<usize> },
    Write { address: usize, value: u64 }
}

struct Program {
    memory: HashMap<usize, u64>,
    ones: u64,
    zeros: u64,
    floating: Vec<usize>,
}

impl Program {
    pub fn new() -> Self {
        Program { memory: HashMap::new(), ones: 0, zeros: 0, floating: Vec::new() }
    }

    fn set_mask(&mut self, ones: u64, zeros: u64, floating: &Vec<usize>) {
        self.ones = ones;
        self.zeros = !zeros;
        self.floating = floating.clone();
    }

    fn write_value_with_mask(&mut self, address: usize, value: u64) {
        let mem = (value | self.ones) & self.zeros;
        self.memory.insert(address, mem);
    }

    fn write_address_with_mask(&mut self, address: usize, value: u64) {
        self.floating.iter()
            .fold(vec![address | self.ones as usize], |acc, index| {
                acc.iter()
                    .flat_map(|&prev| vec![prev | (1 << (35 - index)), prev & !(1 << (35 - index))])
                    .collect::<Vec<_>>()
            })
            .iter()
            .for_each(|&address| { self.memory.insert(address, value); });
    }

    pub fn handle_memory_mask(&mut self, value: &Value) {
        match value {
            Value::Mask { ones, zeros, floating } => self.set_mask(*ones, *zeros, floating),
            Value::Write { address, value } => self.write_value_with_mask(*address, *value)
        }
    }

    pub fn handle_address_mask(&mut self, value: &Value) {
        match value {
            Value::Mask { ones, zeros, floating } => self.set_mask(*ones, *zeros, floating),
            Value::Write { address, value } => self.write_address_with_mask(*address, *value)
        }
    }

    pub fn total(&self) -> u64 {
        self.memory
            .values()
            .sum()
    }
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    let (value, (value_type, _)) = tuple((
        take_until(" = "),
        tag(" = "),
    ))(input)?;
    if value_type == "mask" {
        let (ones, zeros, floating) = value.chars()
            .enumerate()
            .fold((0,0, Vec::new()), |(mut ones, mut zeros, mut floating), (index, next)| {
                ones <<= 1;
                zeros <<= 1;
                match next {
                    '1' =>  ones |= 1, 
                    '0' =>  zeros |= 1,
                    _ => floating.push(index) 
                };
                (ones, zeros, floating)
            });
        return Ok(("", Value::Mask { ones, zeros, floating }));
    } else {
        let (_, (_, address, _)) = tuple((
            tag("mem["),
            map_res(digit1, FromStr::from_str),
            tag("]")
        ))(value_type)?;
        return Ok(("", Value::Write { address, value: FromStr::from_str(value).unwrap() }));
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Value> {
    input.lines()
        .map(|line| parse_value(line))
        .map(|x| x.unwrap())
        .map(|(_, x)| x)
        .collect()
}

#[aoc(day14, part1)]
pub fn solve_part_one(input: &[Value]) -> u64 {
    let mut program = Program::new();
    input.iter().for_each(|v| program.handle_memory_mask(v));
    program.total()
}

#[aoc(day14, part2)]
pub fn solve_part_two(input: &[Value]) -> u64 {
    let mut program = Program::new();
    input.iter().for_each(|v| program.handle_address_mask(v));
    program.total()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT_ONE: &str = indoc! {"
        mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0
    "};

    const INPUT_TWO: &str = indoc! {"
        mask = 000000000000000000000000000000X1001X
        mem[42] = 100
        mask = 00000000000000000000000000000000X0XX
        mem[26] = 1
    "};

    #[test]
    fn it_solves_part_one() {
        let input = input_generator(INPUT_ONE);
        assert_eq!(solve_part_one(&input), 165);
    }
    
   #[test]
   fn it_solves_part_two() {
       let input = input_generator(INPUT_TWO);
       assert_eq!(solve_part_two(&input), 208);
   }

}
