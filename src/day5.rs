
fn parse_boarding_pass(input: &str) -> usize {
    let chars = input.chars();
    let row = chars.clone().take(7)
        .fold((0, 127), |(front, back), char| {
            let half = (front + back) / 2;
            match char {
                'F' => (front, half),
                'B' => (half + 1, back),
                _ => panic!("Invalid input")
            }
        });
    assert!(row.0 == row.1);
    let column = chars.skip(7).take(3)
        .fold((0, 7), |(front, back), char| {
            let half = (front + back) / 2;
            match char {
                'L' => (front, half),
                'R' => (half + 1, back),
                _ => panic!("Invalid input")
            }
        });
    assert!(column.0 == column.1);
    row.0 * 8 + column.0
}

#[aoc(day5, part1)]
pub fn solve_part_one(input: &str) -> usize {
    input.lines()
        .map(parse_boarding_pass)
        .max()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part_two(input: &str) -> usize {
    let mut seats = [false; 127 * 8];
    input.lines()
        .map(parse_boarding_pass)
        .for_each(|seat| seats[seat] = true);
    seats.iter()
        .enumerate()
        .skip_while(|(_, &full)| !full)
        .find(|(_, &full)| !full)
        .unwrap()
        .0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_solves_part_one() {
        assert_eq!(parse_boarding_pass("BFFFBBFRRR"), 567);
        assert_eq!(parse_boarding_pass("FFFBBBFRRR"), 119);
        assert_eq!(parse_boarding_pass("BBFFBBFRLL"), 820);
    }

}
