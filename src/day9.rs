use std::str::FromStr;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.lines()
        .map(FromStr::from_str)
        .filter_map(Result::ok)
        .collect()
}

fn find_invalid_value(input: &[usize], preamble: usize) -> usize {
    let mut upper = preamble;
    while upper < input.len() - 1 {
        let lower = upper - preamble;
        let next = input[upper];
        let is_valid  = input[lower..upper].iter()
            .any(|x| {
                input[lower..upper].iter()
                    .filter(|&y| y != x)
                    .any(|&y| x + y == next)
            });
        if !is_valid { return next }
        upper += 1;
    }
    panic!("No value found")
}

fn find_continguous(input: &[usize], preabmle: usize) -> usize {
    let target = find_invalid_value(input, preabmle);
    let mut lower = 0;
    while lower < input.len() - 2 {
        let mut upper = lower + 1;
        let mut sum = input[lower];
        while sum < target && upper < input.len() {
            sum += input[upper];
            if sum == target {
                let min = input[lower..=upper].iter().min().unwrap();
                let max = input[lower..=upper].iter().max().unwrap();
                return min + max;
            }
            upper += 1;
        }
        lower += 1;
    }
    panic!("No value found")
}

#[aoc(day9, part1)]
pub fn solve_part_one(input: &[usize]) -> usize {
    find_invalid_value(input, 25)
}

#[aoc(day9, part2)]
pub fn solve_part_two(input: &[usize]) -> usize {
    find_continguous(input, 25)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576
    "};

    #[test]
    fn it_solves_part_one() {
        let input = input_generator(INPUT);
        assert_eq!(find_invalid_value(&input, 5), 127);
    }
    
    #[test]
    fn it_solves_part_two() {
        let input = input_generator(INPUT);
        assert_eq!(find_continguous(&input, 5), 62);
    }

}
