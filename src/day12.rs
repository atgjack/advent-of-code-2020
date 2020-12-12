use std::str::FromStr;

use nom::{IResult, character::complete::anychar, character::complete::{line_ending, not_line_ending}, combinator::{map_res, opt}, multi::many1, sequence::tuple};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CardinalDirection {
    North,
    South,
    East,
    West,
}

pub enum Direction {
    Left,
    Right
}

pub enum Action {
    Move(CardinalDirection, usize),
    Turn(Direction, usize),
    Forward(usize)
}

trait Actor {
    fn new() -> Self;
    fn position(&self) -> (isize, isize);
    fn travel(&mut self, direction: Option<CardinalDirection>, amount: &usize);
    fn rotate(&mut self, direction: &Direction, amount: &usize);

    fn distance(&self) -> usize {
        let (x, y) = self.position();
        (x.abs() + y.abs()) as usize
    }

    fn act(&mut self, action: &Action) {
        match action {
            Action::Move(direction, amount) => self.travel(Some(*direction), amount),
            Action::Turn(direction, amount) => self.rotate(direction, amount),
            Action::Forward(amount) => self.travel(None, amount)
        }
    }
}

struct SimpleShip {
    direction: CardinalDirection,
    x: isize,
    y: isize
}

impl Actor for SimpleShip {
    fn new() -> Self {
        SimpleShip { direction: CardinalDirection::East, x: 0, y: 0 }
    }

    fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    fn travel(&mut self, direction: Option<CardinalDirection>, amount: &usize) {
        match direction {
            Some(CardinalDirection::North) => { self.y += *amount as isize; }
            Some(CardinalDirection::South) => { self.y -= *amount as isize; }
            Some(CardinalDirection::East) => { self.x += *amount as isize; }
            Some(CardinalDirection::West) => { self.x -= *amount as isize; }
            None => self.travel(Some(self.direction), amount)
        }
    }

    fn rotate(&mut self, direction: &Direction, amount: &usize) {
        const ORDER: [CardinalDirection; 4] = [
            CardinalDirection::North,
            CardinalDirection::East,
            CardinalDirection::South,
            CardinalDirection::West,
        ];
        let amount = (amount % 360) / 90;
        let amount = match direction {
            Direction::Left => 4 - amount,
            Direction::Right => amount
        };
        self.direction = ORDER.iter()
            .cycle()
            .skip_while(|&dir| self.direction != *dir)
            .skip(amount)
            .next()
            .map(|x| *x)
            .unwrap();
    }
}

struct WaypointShip {
    waypoint_x: isize,
    waypoint_y: isize,
    x: isize,
    y: isize
}

impl Actor for WaypointShip {
    fn new() -> Self {
        WaypointShip { waypoint_x: 10, waypoint_y: 1, x: 0, y: 0 }
    }

    fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    fn travel(&mut self, direction: Option<CardinalDirection>, amount: &usize) {
        match direction {
            Some(CardinalDirection::North) => { self.waypoint_y += *amount as isize; }
            Some(CardinalDirection::South) => { self.waypoint_y -= *amount as isize; }
            Some(CardinalDirection::East) => { self.waypoint_x += *amount as isize; }
            Some(CardinalDirection::West) => { self.waypoint_x -= *amount as isize; }
            None => (0..*amount).for_each(|_| {
                self.x += self.waypoint_x;
                self.y += self.waypoint_y;
            })
        }
    }

    fn rotate(&mut self, direction: &Direction, amount: &usize) {
        let times = (amount % 360) / 90;
        (0..times).for_each(|_| match direction {
            Direction::Left => {
                let (prev_x, prev_y) = (self.waypoint_x, self.waypoint_y);
                self.waypoint_x = -prev_y;
                self.waypoint_y = prev_x;
            }
            Direction::Right => {
                let (prev_x, prev_y) = (self.waypoint_x, self.waypoint_y);
                self.waypoint_x = prev_y;
                self.waypoint_y = -prev_x;
            }
        })
    }
}

fn parse_action(input: &str) -> IResult<&str, Action> {
    let (rest, (c, n, _)) = tuple((
        anychar,
        map_res(not_line_ending, FromStr::from_str),
        opt(line_ending),
    ))(input)?;
    let action = match c {
        'N' => Action::Move(CardinalDirection::North, n),
        'S' => Action::Move(CardinalDirection::South, n),
        'E' => Action::Move(CardinalDirection::East, n),
        'W' => Action::Move(CardinalDirection::West, n),
        'L' => Action::Turn(Direction::Left, n),
        'R' => Action::Turn(Direction::Right, n),
        'F' => Action::Forward(n),
        _ => panic!("Uknown action")
    };
    Ok((rest, action))
}

fn solve<A: Actor>(input: &[Action]) -> usize {
    let mut actor = A::new();
    input.iter().for_each(|action| actor.act(action));
    actor.distance()
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Action> {
    many1(parse_action)(input).unwrap().1
}

#[aoc(day12, part1)]
pub fn solve_part_one(input: &[Action]) -> usize {
    solve::<SimpleShip>(input)
}

#[aoc(day12, part2)]
pub fn solve_part_two(input: &[Action]) -> usize {
    solve::<WaypointShip>(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        F10
        N3
        F7
        R90
        F11
    "};

    #[test]
    fn it_solves_part_one() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part_one(&input), 25);
    }
    
    #[test]
    fn it_solves_part_two() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part_two(&input), 286);
    }

}
