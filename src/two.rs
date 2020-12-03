use crate::day::Day;

struct Input<'a> {
    first: usize,
    second: usize,
    character: char,
    password: &'a str
}

impl <'a> Input<'a> {

    pub fn from_line(line: &'a str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let count_parts: Vec<&str> = parts[0].split('-').collect();
        let first = count_parts[0].parse::<usize>().unwrap();
        let second = count_parts[1].parse::<usize>().unwrap();
        let character = parts[1].split(':').next().unwrap().chars().next().unwrap();
        let password = parts[2];
        Input { first, second, character, password }
    }

    pub fn has_proper_character_count(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.character).count();
        count >= self.first && count <= self.second
    }

    pub fn has_proper_character_position(&self) -> bool {
        let first = self.password.chars().skip(self.first - 1).next();
        let second = self.password.chars().skip(self.second - 1).next();
        match (first, second) {
            (Some(first), Some(second)) => {
                (first == self.character || second == self.character) && first != second
            },
            (Some(first), None) => first == self.character,
            _ => false
        }
    }

}

pub struct DayTwo<'a> {
    items: Vec<Input<'a>>
}

impl <'a> DayTwo<'a> {
    pub fn new(input: &'a str) -> Self {
        let items: Vec<Input> = input.lines()
            .map(Input::from_line)
            .collect();
        DayTwo { items }
    }

    pub fn count_valid_items_with_count(&self) -> usize {
        self.items.iter()
            .filter(|i| i.has_proper_character_count())
            .count()
    }

    pub fn count_valid_items_with_position(&self) -> usize {
        self.items.iter()
            .filter(|i| i.has_proper_character_position())
            .count()
    }
}

impl <'a> Day<usize, usize> for DayTwo<'a> {

    fn part_a(&self) -> usize {
        self.count_valid_items_with_count()
    }

    fn part_b(&self) -> usize {
        self.count_valid_items_with_position()
    }

}

#[test]
fn example_one() {
    let day = DayTwo::new(include_str!("input/two_test.txt"));
    assert_eq!(day.part_a(), 2);
}

#[test]
fn example_two() {
    let day = DayTwo::new(include_str!("input/two_test.txt"));
    assert_eq!(day.part_b(), 1);
}