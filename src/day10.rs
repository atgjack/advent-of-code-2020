use std::{str::FromStr, collections::HashMap};

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<usize> {
    let mut result = input.lines()
        .map(FromStr::from_str)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    result.push(0);
    result.sort();
    result
}

fn recurse(input: &[usize], (one, two, three): (usize, usize, usize)) -> Option<(usize, usize, usize)> {
    if input.len() == 1 { return Some((one, two, three)) }
    match input[1] - input[0] {
        3 => recurse(&input[1..], (one, two, three + 1)),
        2 => recurse(&input[1..], (one, two + 1, three)),
        1 => recurse(&input[1..], (one + 1, two, three)),
        0 => recurse(&input[1..], (one, two, three)),
        _ => None
    }
}

#[aoc(day10, part1)]
pub fn solve_part_one(input: &[usize]) -> usize {
    match recurse(&input, (0, 0, 0)) {
        Some((one, _, three)) => one * (three + 1),
        None => panic!("Value not found")
    }
}

#[aoc(day10, part2)]
pub fn solve_part_two(input: &[usize]) -> usize {
    let mut hashmap: HashMap<usize, usize> = HashMap::new();
    let last = input.iter().last().unwrap();
    hashmap.insert(last + 3, 1);
    input.iter()
        .rev()
        .fold(&mut hashmap, |acc, item| {
            let sum = (1..=3)
                .map(|i| item + i)
                .filter_map(|i| acc.get(&i))
                .sum();
            acc.insert(*item, sum);
            acc
        })
        .get(&0)
        .unwrap()
        .clone()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT_ONE: &str = indoc! {"
        16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4
    "};

    const INPUT_TWO: &str = indoc! {"
        28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3
    "};

    #[test]
    fn it_solves_part_one() {
        let input_one = input_generator(INPUT_ONE);
        let input_two = input_generator(INPUT_TWO);
        assert_eq!(solve_part_one(&input_one), 7 * 5);
        assert_eq!(solve_part_one(&input_two), 22 * 10);
    }
    
    #[test]
    fn it_solves_part_two() {
        let input_one = input_generator(INPUT_ONE);
        let input_two = input_generator(INPUT_TWO);
        assert_eq!(solve_part_two(&input_one), 8);
        assert_eq!(solve_part_two(&input_two), 19208);
    }

}
