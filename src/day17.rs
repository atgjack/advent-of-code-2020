use std::collections::HashSet;

const NEIGBOR_COMBINATIONS_3D: [Coordinate<3>; 27] = [
    [-1, -1, -1],
    [-1, -1,  0],
    [-1, -1,  1],
    [-1,  0, -1],
    [-1,  0,  0],
    [-1,  0,  1],
    [-1,  1, -1],
    [-1,  1,  0],
    [-1,  1,  1],
    [ 0, -1, -1],
    [ 0, -1,  0],
    [ 0, -1,  1],
    [ 0,  0, -1],
    [ 0,  0,  0],
    [ 0,  0,  1],
    [ 0,  1, -1],
    [ 0,  1,  0],
    [ 0,  1,  1],
    [ 1, -1, -1],
    [ 1, -1,  0],
    [ 1, -1,  1],
    [ 1,  0, -1],
    [ 1,  0,  0],
    [ 1,  0,  1],
    [ 1,  1, -1],
    [ 1,  1,  0],
    [ 1,  1,  1],
];

const NEIGBOR_COMBINATIONS_4D: [Coordinate<4>; 81] = [
    [-1, -1, -1, -1],
    [-1, -1, -1,  0],
    [-1, -1, -1,  1],
    [-1, -1,  0, -1],
    [-1, -1,  0,  0],
    [-1, -1,  0,  1],
    [-1, -1,  1, -1],
    [-1, -1,  1,  0],
    [-1, -1,  1,  1],
    [-1,  0, -1, -1],
    [-1,  0, -1,  0],
    [-1,  0, -1,  1],
    [-1,  0,  0, -1],
    [-1,  0,  0,  0],
    [-1,  0,  0,  1],
    [-1,  0,  1, -1],
    [-1,  0,  1,  0],
    [-1,  0,  1,  1],
    [-1,  1, -1, -1],
    [-1,  1, -1,  0],
    [-1,  1, -1,  1],
    [-1,  1,  0, -1],
    [-1,  1,  0,  0],
    [-1,  1,  0,  1],
    [-1,  1,  1, -1],
    [-1,  1,  1,  0],
    [-1,  1,  1,  1],
    [ 0, -1, -1, -1],
    [ 0, -1, -1,  0],
    [ 0, -1, -1,  1],
    [ 0, -1,  0, -1],
    [ 0, -1,  0,  0],
    [ 0, -1,  0,  1],
    [ 0, -1,  1, -1],
    [ 0, -1,  1,  0],
    [ 0, -1,  1,  1],
    [ 0,  0, -1, -1],
    [ 0,  0, -1,  0],
    [ 0,  0, -1,  1],
    [ 0,  0,  0, -1],
    [ 0,  0,  0,  0],
    [ 0,  0,  0,  1],
    [ 0,  0,  1, -1],
    [ 0,  0,  1,  0],
    [ 0,  0,  1,  1],
    [ 0,  1, -1, -1],
    [ 0,  1, -1,  0],
    [ 0,  1, -1,  1],
    [ 0,  1,  0, -1],
    [ 0,  1,  0,  0],
    [ 0,  1,  0,  1],
    [ 0,  1,  1, -1],
    [ 0,  1,  1,  0],
    [ 0,  1,  1,  1],
    [ 1, -1, -1, -1],
    [ 1, -1, -1,  0],
    [ 1, -1, -1,  1],
    [ 1, -1,  0, -1],
    [ 1, -1,  0,  0],
    [ 1, -1,  0,  1],
    [ 1, -1,  1, -1],
    [ 1, -1,  1,  0],
    [ 1, -1,  1,  1],
    [ 1,  0, -1, -1],
    [ 1,  0, -1,  0],
    [ 1,  0, -1,  1],
    [ 1,  0,  0, -1],
    [ 1,  0,  0,  0],
    [ 1,  0,  0,  1],
    [ 1,  0,  1, -1],
    [ 1,  0,  1,  0],
    [ 1,  0,  1,  1],
    [ 1,  1, -1, -1],
    [ 1,  1, -1,  0],
    [ 1,  1, -1,  1],
    [ 1,  1,  0, -1],
    [ 1,  1,  0,  0],
    [ 1,  1,  0,  1],
    [ 1,  1,  1, -1],
    [ 1,  1,  1,  0],
    [ 1,  1,  1,  1],
];

type Coordinate<const N: usize> = [isize; N];

trait Conway<const N: usize> {
    fn get_neighbors(coord: &Coordinate<N>) -> HashSet<Coordinate<N>>;

    fn translate_input(input: &[Coordinate<2>]) -> HashSet<Coordinate<N>> {
        input.into_iter()
            .map(|&[x, y]| {
                let mut res = [0; N];
                res[0] = x;
                res[1] = y;
                res
            })
            .collect()
    }

    fn evolve(input: HashSet<Coordinate<N>>, _: usize) -> HashSet<Coordinate<N>> {
        let active: HashSet<Coordinate<N>> = input.into_iter().collect();
        active.iter()
            .flat_map(<Self as Conway<N>>::get_neighbors)
            .filter(|coord| {
                let active_neighbors = <Self as Conway<N>>::get_neighbors(coord)
                    .into_iter()
                    .filter(|neighbor| active.contains(neighbor))
                    .count();
                if active.contains(coord) {
                    active_neighbors == 3 || active_neighbors == 4
                } else {
                    active_neighbors == 3
                }
            })
            .collect()
    }
}

impl Conway<3> for Coordinate<3> {
    fn get_neighbors(coord: &Coordinate<3>) -> HashSet<Coordinate<3>> {
        NEIGBOR_COMBINATIONS_3D
            .iter()
            .map(|perm| [coord[0] + perm[0], coord[1] + perm[1], coord[2] + perm[2]])
            .collect()
    }
}

impl Conway<4> for Coordinate<4> {
    fn get_neighbors(coord: &Coordinate<4>) -> HashSet<Coordinate<4>> {
        NEIGBOR_COMBINATIONS_4D
            .iter()
            .map(|perm| [coord[0] + perm[0], coord[1] + perm[1], coord[2] + perm[2], coord[3] + perm[3]])
            .collect()
    }
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<Coordinate<2>> {
    input.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| match c {
                    '.' => None,
                    '#' => Some([x as isize, y as isize]), 
                    _ => panic!("Unexpected character")
                })
        })
        .collect()
}

#[aoc(day17, part1)]
pub fn solve_part_one(input: &[Coordinate<2>]) -> usize {
    let start = <Coordinate<3> as Conway<3>>::translate_input(input);
    (0..6usize)
        .fold(start, <Coordinate<3> as Conway<3>>::evolve)
        .len()
}

#[aoc(day17, part2)]
pub fn solve_part_two(input: &[Coordinate<2>]) -> usize {
    let start = <Coordinate<4> as Conway<4>>::translate_input(input);
    (0..6usize)
        .fold(start, <Coordinate<4> as Conway<4>>::evolve)
        .len()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        .#.
        ..#
        ###
    "};

    #[test]
    fn it_solves_part_one() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part_one(&input), 112);
    }
    
    #[test]
    fn it_solves_part_two() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part_two(&input), 848);
    }

}
