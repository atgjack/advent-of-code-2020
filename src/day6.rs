use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn solve_part_one(input: &str) -> usize {
    let mut output = Vec::new();
    output.push(HashSet::new());
    let lines = input.lines();
    
    for line in lines {
        if line.is_empty() {
            output.push(HashSet::new());
            continue;
        }
        let current = output.last_mut().unwrap();
        line.chars().for_each(|x| { current.insert(x); });
    }

    if output.last().unwrap().is_empty() { output.pop(); }
    
    output.iter()
        .map(HashSet::len)
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part_two(input: &str) -> usize {
    let mut output = Vec::new();
    output.push(HashSet::new());
    let lines = input.lines();
    let mut is_new = true;
    
    for line in lines {
        if line.is_empty() {
            output.push(HashSet::new());
            is_new = true;
            continue;
        }
        let current = output.pop().unwrap();
        let chars: HashSet<char> = line.chars().collect();
        if is_new {
            output.push(chars);
            is_new = false;
        } else {
            let intersection: HashSet<char> = current.intersection(&chars)
                .map(|x| *x)
                .collect(); 
            output.push(intersection);
        }
        
    }

    if output.last().unwrap().is_empty() { output.pop(); }
    
    output.iter()
        .map(HashSet::len)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        abc

        a
        b
        c
        
        ab
        ac
        
        a
        a
        a
        a
        
        b
    "};

    #[test]
    fn it_solves_part_one() {
        assert_eq!(solve_part_one(&INPUT), 11);
    }
    
    #[test]
    fn it_solves_part_two() {
        assert_eq!(solve_part_two(&INPUT), 6);
    }

}
