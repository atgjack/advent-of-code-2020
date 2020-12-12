use std::{cell::RefCell, collections::{HashMap, HashSet}, iter::repeat};

type Position = (isize, isize);
type Grid = HashMap<Position, Space>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Space {
    Occupied,
    Empty,
    Floor
}

impl Space {
    pub fn from_char(c: char) -> Self {
        match c {
            '#' => Space::Occupied,
            'L' => Space::Empty,
            '.' => Space::Floor,
            _ => panic!("Invalid character")
        }
    }
}

const DIRECTIONS: [[isize;2]; 8] = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]]; 

trait Heuristic {
    fn will_change(grid: &Grid, pos: &Position) -> bool;
    fn affected_by(grid: &Grid, pos: &Position) -> HashSet<Position>;
}

struct Neighbors {}

impl Heuristic for Neighbors {
    fn will_change(grid: &Grid, pos: &Position) -> bool {
        let occupied_adjecent = Self::affected_by(grid, pos)
            .iter()
            .filter_map(|next: &Position| grid.get(&next))
            .filter(|&space| *space == Space::Occupied)
            .count();
        match grid.get(&pos) {
            Some(Space::Occupied) => occupied_adjecent >= 4,
            Some(Space::Empty) => occupied_adjecent == 0,
            Some(Space::Floor) => false,
            None => false
        }
    }

    fn affected_by(grid: &Grid, pos: &Position) -> HashSet<Position> {
        DIRECTIONS
            .iter()
            .map(|[x, y]| (pos.0 + x, pos.1 + y))
            .filter(|next| grid.contains_key(next))
            .collect()
    }
}

struct LineOfSight {}

impl Heuristic for LineOfSight {
    fn will_change(grid: &Grid, pos: &Position) -> bool {
        let occupied_adjecent = Self::affected_by(grid, pos)
            .iter()
            .filter_map(|next: &Position| grid.get(&next))
            .filter(|&space| *space == Space::Occupied)
            .count();
        match grid.get(&pos) {
            Some(Space::Occupied) => occupied_adjecent >= 5,
            Some(Space::Empty) => occupied_adjecent == 0,
            Some(Space::Floor) => false,
            None => false
        }
    }

    fn affected_by(grid: &Grid, pos: &Position) -> HashSet<Position> {
        DIRECTIONS
            .iter()
            .filter_map(|dir| {
                repeat(dir)
                    .scan(RefCell::new(*pos), |state, [x, y]| {
                        let state = state.get_mut();
                        state.0 = state.0 + x;
                        state.1 = state.1 + y;
                        Some((state.0, state.1))
                    })
                    .take_while(|pos| grid.contains_key(pos))
                    .find(|pos| match grid.get(pos) {
                        Some(Space::Floor) => false,
                        _ => true
                    })
            })
            .collect()
    }
}

fn get_changes<H: Heuristic>(grid: &Grid, prev_changes: &HashSet<Position>) -> HashSet<Position> {
    let prev_affected = prev_changes.iter()
        .flat_map(|pos| H::affected_by(grid, pos))
        .collect::<HashSet<_>>();
    prev_affected.iter()
        .filter(|&pos| H::will_change(grid, pos))
        .map(|&x| x)
        .collect()
}

fn apply_changes(grid: &mut Grid, changes: &HashSet<Position>) {
    changes.iter()
        .for_each(|pos| {
            let next = match grid.get(pos).unwrap() {
                Space::Empty => Space::Occupied,
                Space::Occupied => Space::Empty,
                Space::Floor => return,
            };
            grid.insert(*pos, next);
        });
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Grid {
    input.lines()
        .enumerate()
        .fold(HashMap::new(), |hashmap, (x, line)| {
            line.chars()
                .enumerate()
                .fold(hashmap, |mut acc, (y, c)| {
                    acc.insert((x as isize, y as isize), Space::from_char(c));
                    acc
                })
        })
}

fn solve<H: Heuristic>(input: &Grid) -> usize {
    let mut grid = (*input).clone();
    let mut changes = input.iter()
        .filter(|&(_, space)| *space != Space::Floor)
        .map(|(pos,_)| *pos)
        .collect::<HashSet<_>>();
    while !changes.is_empty() {
        changes = get_changes::<H>(&grid, &changes);
        apply_changes(&mut grid, &changes);
    }
    grid.values()
        .filter(|&v| *v == Space::Occupied)
        .count()
}

#[aoc(day11, part1)]
pub fn solve_part_one(input: &Grid) -> usize {
    solve::<Neighbors>(input)
}

#[aoc(day11, part2)]
pub fn solve_part_two(input: &Grid) -> usize {
    solve::<LineOfSight>(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        #.##.##.##
        #######.##
        #.#.#..#..
        ####.##.##
        #.##.##.##
        #.#####.##
        ..#.#.....
        ##########
        #.######.#
        #.#####.##
    "};

    #[test]
    fn it_solves_part_one() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part_one(&input), 37);
    }
    
    #[test]
    fn it_solves_part_two() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part_two(&input), 26);
    }

}
