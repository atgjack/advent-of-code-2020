use std::str::FromStr;

use nom::{IResult, bytes::complete::take, character::complete::not_line_ending, character::complete::{line_ending, space1}, combinator::map_res, multi::many1, sequence::tuple};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OpCode {
    Nop(isize),
    Acc(isize),
    Jmp(isize)
}

fn parse_opcode(input: &str) -> IResult<&str, OpCode> {
    let result: IResult<&str,(_, _, isize, _)> = tuple((
        take(3u8),
        space1,
        map_res(not_line_ending, FromStr::from_str),
        line_ending
    ))(input);
    match result {
        Ok((rest, ("nop", _, value, _))) => Ok((rest, OpCode::Nop(value))),
        Ok((rest, ("jmp", _, value, _))) => Ok((rest, OpCode::Jmp(value))),
        Ok((rest, ("acc", _, value, _))) => Ok((rest, OpCode::Acc(value))),
        Err(e) => Err(e),
        _ => panic!("Invalid input...")
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<OpCode> {
    many1(parse_opcode)(input).unwrap().1
}

pub fn execute(input: &[OpCode]) -> (isize, bool) {
    let mut visited = Vec::new();
    let mut acc = 0;
    let mut ptr = 0;
    loop {
        let instr = &input[ptr];
        match instr {
            OpCode::Nop(_) => {
                ptr += 1;
            },
            OpCode::Acc(value) => {
                acc += value;
                ptr += 1;
            }
            OpCode::Jmp(value) => {
                ptr = (ptr as isize + value) as usize
            }
        }
        if ptr >= input.len() { return (acc, true) }
        if visited.contains(&ptr) { return (acc, false) }
        visited.push(ptr)
    }
}

pub fn find_permutations(input: &[OpCode]) -> Vec<(usize, OpCode)> {
    input.iter()
        .enumerate()
        .filter_map(|(index, &op)| match op {
            OpCode::Nop(value) => Some((index, OpCode::Jmp(value))),
            OpCode::Jmp(value) => Some((index, OpCode::Nop(value))),
            _ => None
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part_one(input: &[OpCode]) -> isize {
    execute(input).0
}

#[aoc(day8, part2)]
pub fn solve_part_two(input: &[OpCode]) -> isize {
    let mut input = input.iter().map(|x| *x).collect::<Vec<_>>();
    for (index, op) in find_permutations(&input) {
        let old_op = input[index];
        input[index] = op;
        match execute(&input) {
            (value, true) => { return value },
            _ => {}
        }
        input[index] = old_op;
    }
    panic!("No value found");
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6
    "};

    #[test]
    fn it_solves_part_one() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part_one(&input), 5);
    }
    
    #[test]
    fn it_solves_part_two() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part_two(&input), 8);
    }

}
