use std::str::FromStr;

fn calculate_next_time(bus_time: &usize, current_time: &usize) -> usize {
    if current_time % bus_time == 0 { return 0 }
    (((current_time / bus_time) + 1) * bus_time) - current_time
}

fn calculate_wait(bus_time: usize, offset: usize, time: usize, wait_time: usize) -> (usize, usize) {
    let next_overlap = (time..)
        .step_by(wait_time)
        .find(|next_overlap| (next_overlap + offset) % bus_time == 0)
        .unwrap();
    (next_overlap, wait_time * bus_time)
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (usize, Vec<Option<usize>>) {
    let mut lines = input.lines();
    let time = lines.next().unwrap().parse().unwrap();
    let busses = lines.next().unwrap()
        .split(",")
        .map(FromStr::from_str)
        .map(Result::ok)
        .collect();
    (time, busses)
}

#[aoc(day13, part1)]
pub fn solve_part_one((time, busses): &(usize, Vec<Option<usize>>)) -> usize {
    let (index, diff) = busses.iter()
        .enumerate()
        .filter_map(|(index, bus)| bus.map(|bus| (index, bus)))
        .map(|(index, bus)| (index, calculate_next_time(&bus, time)))
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();
    busses[index].unwrap() * diff
}

#[aoc(day13, part2)]
pub fn solve_part_two((_, busses): &(usize, Vec<Option<usize>>)) -> usize {
    busses.iter()
        .enumerate()
        .filter_map(|(index, bus)| bus.map(|bus| (index, bus)))
        .fold((0, 1), |(time, wait), (index, bus)| calculate_wait(bus, index, time, wait))
        .0
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT_ONE: &str = indoc! {"
        939
        7,13,x,x,59,x,31,19
    "};

    const INPUT_TWO: &str = indoc! {"
        939
        17,x,13,19
    "};

    const INPUT_THREE: &str = indoc! {"
        939
        67,7,59,61
    "};

    const INPUT_FOUR: &str = indoc! {"
        939
        67,x,7,59,61
    "};

    const INPUT_FIVE: &str = indoc! {"
        939
        67,7,x,59,61
    "};

    const INPUT_SIX: &str = indoc! {"
        939
        1789,37,47,1889
    "};

    #[test]
    fn it_solves_part_one() {
        let input = input_generator(INPUT_ONE);
        assert_eq!(solve_part_one(&input), 295);
    }
    
    #[test]
    fn it_solves_part_two() {
        let one = input_generator(INPUT_ONE);
        let two = input_generator(INPUT_TWO);
        let three = input_generator(INPUT_THREE);
        let four = input_generator(INPUT_FOUR);
        let five = input_generator(INPUT_FIVE);
        let six = input_generator(INPUT_SIX);
        assert_eq!(solve_part_two(&one), 1068781);
        assert_eq!(solve_part_two(&two), 3417);
        assert_eq!(solve_part_two(&three), 754018);
        assert_eq!(solve_part_two(&four), 779210);
        assert_eq!(solve_part_two(&five), 1261476);
        assert_eq!(solve_part_two(&six), 1202161486);
    }

}
