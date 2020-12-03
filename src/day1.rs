
fn recurse(input: &[isize], offset: usize, entries: &mut [isize], depth: usize) -> Option<isize> {
    (offset..input.len())
        .find_map(|offset| {
            
            let sum: isize = entries[..depth].iter().sum();
            if sum > 2020 { return None }

            if depth >= entries.len() {
                return match sum {
                    2020 => Some(entries[..depth].iter().product()),
                    _ => None
                };
            }

            entries[depth] = input[offset];
            recurse(input, offset + 1, entries, depth + 1)

        })
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input.lines()
        .filter_map(|x| x.parse::<isize>().ok())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part_one(input: &[isize]) -> isize {
    recurse(input, 0, &mut [0isize; 2], 0).unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part_two(input: &[isize]) -> isize {
    recurse(input, 0, &mut [0isize; 3], 0).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        1721
        979
        366
        299
        675
        1456
    "};

    #[test]
    fn it_solves_part_one() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part_one(&input), 514579);
    }
    
    #[test]
    fn it_solves_part_two() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part_two(&input), 241861950);
    }

}
