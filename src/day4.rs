use std::{str::FromStr, collections::HashMap};
use nom::{IResult, character::complete::{alpha1, digit1}, combinator::map_res, bytes::complete::tag, sequence::tuple};

type Passport = HashMap<String, String>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
    let mut output = Vec::new();
    output.push(HashMap::new());
    let lines = input.lines();
    
    for line in lines {
        if line.is_empty() {
            output.push(HashMap::new());
            continue;
        }
        let current = output.last_mut().unwrap();
        for part in line.split_whitespace() {
            let field_parts: Vec<&str> = part.split(':').collect();
            current.insert(field_parts[0].to_owned(), field_parts[1].to_owned());
        }
    }

    if output.last().unwrap().is_empty() { output.pop(); }
    output
}

const REQUIRED_FIELDS: &'static [&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const EYE_COLORS: &'static [&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn year(input: &str, min: usize, max: usize) -> bool {
    let result: IResult<&str, usize> = map_res(digit1, FromStr::from_str)(input);
    match result {
        Ok((_, year)) => year >= min && year <= max,
        _ => false
    }
}

fn height(input: &str) -> bool {
    let result: IResult<&str, (usize, &str)> = tuple(( map_res(digit1, FromStr::from_str), alpha1 ))(input);
    match result {
        Ok((_, (num, "cm"))) => num >= 150 && num <= 193,
        Ok((_, (num, "in"))) => num >= 59 && num <= 76,
        _ => false
    }
}

fn hex(input: &str) -> bool {
    let result: IResult<&str, &str> = tag("#")(input);
    match result {
        Ok((value, _)) => {
            if value.len() != 6 { return false }
            value.chars().all(|c| c.is_digit(16))
        },
        Err(_) => false
    }
}

fn eye_color(input: &str) -> bool {
    EYE_COLORS.iter().any(|&v| v == input)
}

fn pid(input: &str) -> bool {
    if input.len() != 9 { return false }
    input.chars().all(|c| c.is_digit(10))
}

pub fn contains_all_fields(passport: &&Passport) -> bool {
    REQUIRED_FIELDS.iter().all(|f| passport.contains_key(*f))
}

pub fn validate_fields(passport: &&Passport) -> bool {
    passport.iter()
        .all(|(key, value)| {
            match key.as_str() {
                "byr" => year(value, 1920, 2002),
                "iyr" => year(value, 2010, 2020),
                "eyr" => year(value, 2020, 2030),
                "hgt" => height(value),
                "hcl" => hex(value),
                "ecl" => eye_color(value),
                "pid" => pid(value),
                "cid" => true,
                _ => false
            }
        })
}

#[aoc(day4, part1)]
pub fn solve_part_one(input: &[Passport]) -> usize {
    input.iter()
        .filter(contains_all_fields)
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part_two(input: &[Passport]) -> usize {
    input.iter()
        .filter(contains_all_fields)
        .filter(validate_fields)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm
        
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929
        
        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm
        
        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in
    "};

    const INVALID: &str = indoc! {"
        eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
        
        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946
        
        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
        
        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007
    "};

    const VALID: &str = indoc! {"
        pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        hcl:#623a2f
        
        eyr:2029 ecl:blu cid:129 byr:1989
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
        
        hcl:#888785
        hgt:164cm byr:2001 iyr:2015 cid:88
        pid:545766238 ecl:hzl
        eyr:2022
        
        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
    "};

    #[test]
    fn it_solves_part_one() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part_one(&input), 2);
    }
    
    #[test]
    fn it_solves_part_two() {
        let valid_input = input_generator(VALID);
        let invalid_input = input_generator(INVALID);
        assert_eq!(solve_part_two(&valid_input), 4);
        assert_eq!(solve_part_two(&invalid_input), 0);
    }

}
