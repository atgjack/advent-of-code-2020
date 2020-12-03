
pub struct Rule {
    first: usize,
    second: usize,
    character: char,
    password: String
}

impl Rule {

    pub fn from_line(line: &str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let count_parts: Vec<&str> = parts[0].split('-').collect();
        let first = count_parts[0].parse::<usize>().unwrap();
        let second = count_parts[1].parse::<usize>().unwrap();
        let character = parts[1].split(':').next().unwrap().chars().next().unwrap();
        let password = parts[2].to_owned();
        Rule { first, second, character, password }
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

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Rule> {
    input.lines()
        .map(Rule::from_line)
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part_one(input: &[Rule]) -> usize {
    input.iter()
        .filter(|i| i.has_proper_character_count())
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part_two(input: &[Rule]) -> usize {
    input.iter()
        .filter(|i| i.has_proper_character_position())
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        1-3 a: abcde
        1-3 b: cdefg
        2-9 c: ccccccccc
    "};

    #[test]
    fn it_solves_part_one() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part_one(&input), 2);
    }
    
    #[test]
    fn it_solves_part_two() {
        let input = input_generator(INPUT);
        assert_eq!(solve_part_two(&input), 1);
    }

}