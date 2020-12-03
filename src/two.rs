use crate::day::Day;

struct Input<'a> {
    min: usize,
    max: usize,
    character: char,
    password: &'a str
}

impl <'a> Input<'a> {

    pub fn from_line(line: &'a str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let count_parts: Vec<&str> = parts[0].split('-').collect();
        let min = count_parts[0].parse::<usize>().unwrap();
        let max = count_parts[1].parse::<usize>().unwrap();
        let character = parts[1].split(':').next().unwrap().chars().next().unwrap();
        let password = parts[2];
        Input { min, max, character, password }
    }

    pub fn is_valid(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.character).count();
        count >= self.min && count <= self.max
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

    pub fn count_valid_items(&self) -> usize {
        self.items.iter()
            .filter(|i| i.is_valid())
            .count()
    }
}

impl <'a> Day<usize, usize> for DayTwo<'a> {

    fn part_a(&self) -> usize {
        self.count_valid_items()
    }

    fn part_b(&self) -> usize {
        todo!()
    }

}

#[test]
fn example_one() {
    let day = DayTwo::new(include_str!("input/two_test.txt"));
    assert_eq!(day.part_a(), 2);
}