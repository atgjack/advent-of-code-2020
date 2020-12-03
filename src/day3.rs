type Grid = Vec<Vec<bool>>;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input.lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn count_trees(input: &Grid, (rise, run): (usize, usize)) -> usize {
    let mut height = 0;
    let mut position = 0;
    let mut count = 0;
    let width = input[0].len();
    loop {
        position = (position + run) % width;
        height = height + rise;
        if height >= input.len() { break }
        if input[height][position] { count += 1 }
    }
    count
}

#[aoc(day3, part1)]
pub fn solve_part_one(input: &Grid) -> usize {
    count_trees(input, (1,3))
}

#[aoc(day3, part2)]
pub fn solve_part_two(input: &Grid) -> usize {
    let slopes = [
        (1, 1),
        (1, 3),
        (1, 5),
        (1, 7),
        (2, 1),
    ];
    slopes.iter()
        .map(|&slope| count_trees(input, slope))
        .product()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        ..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#
    "};

    #[test]
    fn it_solves_part_one() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part_one(&input), 7);
    }
    
    #[test]
    fn it_solves_part_two() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part_two(&input), 336);
    }

}