use std::str::FromStr;

use nom::{IResult, bytes::complete::tag, bytes::complete::take, character::complete::digit1, character::complete::space1, combinator::map_res, sequence::tuple};


pub struct Rule {
    first: usize,
    second: usize,
    character: char,
    password: String
}

impl Rule {

    pub fn from_line(input: &str) -> Self {
        let result: IResult<&str, (usize, &str, usize, &str, char,  &str, &str)> = tuple((
            map_res(digit1, FromStr::from_str),
            tag("-"),
            map_res(digit1, FromStr::from_str),
            space1,
            map_res(take(1u8), |s: &str| { s.chars().next().ok_or(nom::Err::Failure("Empty")) }),
            tag(":"),
            space1,
        ))(input);
        let (password, (first, _, second, _, character, _, _)) = result.unwrap();
        Rule { first, second, character, password: password.to_owned() }
    }

    pub fn has_proper_character_count(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.character).count();
        count >= self.first && count <= self.second
    }

    pub fn has_proper_character_position(&self) -> bool {
        let first = self.password.chars().skip(self.first - 1).next();
        let second = self.password.chars().skip(self.second - 1).next();
        match (first, second) {
            (Some(first), Some(second)) => (first == self.character || second == self.character) && first != second,
            (Some(first), None) => first == self.character,
            _ => false
        }
    }

}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Rule> {
    input.lines()
        .map(Rule::from_line)
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part_one(input: &[Rule]) -> usize {
    input.iter()
        .filter(|i| i.has_proper_character_count())
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part_two(input: &[Rule]) -> usize {
    input.iter()
        .filter(|i| i.has_proper_character_position())
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc
    "};

    #[test]
    fn it_solves_part_one() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part_one(&input), 2);
    }
    
    #[test]
    fn it_solves_part_two() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part_two(&input), 1);
    }

}