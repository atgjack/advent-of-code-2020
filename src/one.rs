use crate::day::Day;

pub struct DayOne {
    numbers: Vec<isize>
}

impl DayOne {
    pub fn new(input: &str) -> Self {
        let numbers: Vec<isize> = input.lines()
            .filter_map(|x| x.parse::<isize>().ok())
            .collect();
        DayOne { numbers }
    }

    fn recurse(&self, offset: usize, entries: &mut [isize], depth: usize) -> Option<isize> {
        (offset..self.numbers.len())
            .find_map(|offset| {
                
                let sum: isize = entries[..depth].iter().sum();
                if sum > 2020 { return None }

                if depth >= entries.len() {
                    return match sum {
                        2020 => Some(entries[..depth].iter().product()),
                        _ => None
                    };
                }

                entries[depth] = self.numbers[offset];
                self.recurse(offset + 1, entries, depth + 1)

            })
    }
    
}

impl Day<isize, isize> for DayOne {

    fn part_a(&self) -> isize {
        self.recurse(0, &mut [0isize; 2], 0).unwrap()
    }

    fn part_b(&self) -> isize {
        self.recurse(0, &mut [0isize; 3], 0).unwrap()
    }

}

#[test]
fn example_one() {
    let day = DayOne::new(include_str!("input/one_test.txt"));
    assert_eq!(day.part_a(), 514579);
}

#[test]
fn example_two() {
    let day = DayOne::new(include_str!("input/one_test.txt"));
    assert_eq!(day.part_b(), 241861950);
}