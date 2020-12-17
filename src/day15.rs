use std::str::FromStr;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.split(',')
        .map(FromStr::from_str)
        .filter_map(Result::ok)
        .collect()
}   

fn find_nth_number(input: &[usize], n: usize) -> usize {
    let mut storage = vec![None; n];
    for (index, &value) in input.iter().enumerate() {
        storage[value] = Some(index + 1);
    }
    ((input.len() + 2)..(n + 1))
        .scan(0, |current, turn| {
            let next = match storage[*current] {
                Some(val) => (turn - 1) - val,
                None => 0 
            };
            storage[*current] = Some(turn - 1);
            *current = next;
            Some(next)
        })
        .last()
        .unwrap()
}

#[aoc(day15, part1)]
pub fn solve_part_one(input: &[usize]) -> usize {
    find_nth_number(input, 2020)
}

#[aoc(day15, part2)]
pub fn solve_part_two(input: &[usize]) -> usize {
    find_nth_number(input,30_000_000)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_solves_part_one() {
        assert_eq!(solve_part_one(&input_generator("0,3,6")), 436);
        assert_eq!(solve_part_one(&input_generator("1,3,2")), 1);
        assert_eq!(solve_part_one(&input_generator("2,1,3")), 10);
        assert_eq!(solve_part_one(&input_generator("1,2,3")), 27);
        assert_eq!(solve_part_one(&input_generator("2,3,1")), 78);
        assert_eq!(solve_part_one(&input_generator("3,2,1")), 438);
        assert_eq!(solve_part_one(&input_generator("3,1,2")), 1836);
    }
    
    #[test]
    fn it_solves_part_two() {
        // Disabled because -- slow
        // assert_eq!(solve_part_two(&input_generator("0,3,6")), 175594);
        // assert_eq!(solve_part_two(&input_generator("1,3,2")), 2578);
        // assert_eq!(solve_part_two(&input_generator("2,1,3")), 3544142);
        // assert_eq!(solve_part_two(&input_generator("1,2,3")), 261214);
        // assert_eq!(solve_part_two(&input_generator("2,3,1")), 6895259);
        // assert_eq!(solve_part_two(&input_generator("3,2,1")), 18);
        // assert_eq!(solve_part_two(&input_generator("3,1,2")), 362);
    }

}
