use std::{collections::HashSet, str::FromStr, collections::HashMap};
use nom::{IResult, bytes::complete::tag, bytes::complete::take_until, character::complete::digit1, character::complete::newline, combinator::map_res, combinator::opt, multi::many1, multi::separated_list1, character::complete::space1};
use nom::sequence::tuple;
use nom::branch::alt;

fn parse_bag(input: &str) -> IResult<&str, Option<Vec<(String, usize)>>> {
    let result: IResult<&str, _> = tag("no other bags")(input);
    match result {
        Ok((rest, _)) => Ok((rest, None)),
        Err(_) => {
            let result: IResult<&str, Vec<(usize, &str, &str, &str)>> = separated_list1(
                tag(", "),
                tuple((
                    map_res(digit1, FromStr::from_str),
                    space1,
                    take_until(" bag"),
                    alt((tag(" bags"), tag(" bag")))
                ))
            )(input);
            match result {
                Err(e) => Err(e), 
                Ok((rest, vec)) => {
                    let types = vec.iter()
                        .map(|(count, _, color, _)| (color.to_string(), *count))
                        .collect();
                    Ok((rest, Some(types)))
                },
            }
        }
    }
}

type Bag = (String, Vec<(String, usize)>);

#[aoc_generator(day7)]
pub fn parse_bags(input: &str) -> Vec<Bag> {
    let mut bags = Vec::new();
    let result: IResult<_, Vec<(&str,&str,Option<Vec<(String, usize)>>, _, _, Option<_>)>> = many1(tuple((
        take_until(" bags contain "),
        tag(" bags contain "),
        parse_bag,
        take_until("."),
        tag("."),
        opt(newline)
    )))(input);
    match result {
        Ok((_, vec)) => vec.iter().for_each(|(color, _, types, _, _, _)| {
            let mut child_colors = Vec::new();
            match types {
                None => {},
                Some(types) => types.iter()
                    .for_each(|(color, count)| child_colors.push((color.to_string(), *count)))
            };
            bags.push((color.to_string(), child_colors));
        }),
        Err(_) => {}
    };
    bags
}

fn find_matching(input: &[Bag], target: &str) -> Vec<String> {
    let direct = input.iter()
        .filter(|(_, children)| children.iter().any(|(color, _)| color == target))
        .map(|(color, _)| color.to_string())
        .collect::<Vec<_>>();
    let nested = direct.iter()
        .flat_map(|color| find_matching(input, color))
        .collect::<Vec<_>>();
    [direct, nested].concat()
}

fn count_needed(input: &HashMap<String, &Vec<(String, usize)>>, target: &str) -> usize {
    match input.get(target) {
        None => 0,
        Some(vec) =>
            vec.iter()
                .map(|(color, count)| { count * count_needed(input, color)})
                .sum::<usize>() + 1
    }
}

const TARGET: &str = "shiny gold";

#[aoc(day7, part1)]
pub fn solve_part_one(input: &[Bag]) -> usize {
    find_matching(input, TARGET)
        .iter()
        .collect::<HashSet<_>>()
        .len()
}

#[aoc(day7, part2)]
pub fn solve_part_two(input: &[Bag]) -> usize {
    let mut hashmap = HashMap::new();
    input.iter().for_each(|(color, children)| { hashmap.insert(color.to_string(), children); });
    count_needed(&hashmap, TARGET) - 1
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.    
    "};

    const INPUT_TWO: &str = indoc! {"
        shiny gold bags contain 2 dark red bags.
        dark red bags contain 2 dark orange bags.
        dark orange bags contain 2 dark yellow bags.
        dark yellow bags contain 2 dark green bags.
        dark green bags contain 2 dark blue bags.
        dark blue bags contain 2 dark violet bags.
        dark violet bags contain no other bags. 
    "};

    #[test]
    fn it_solves_part_one() {
        let input = parse_bags(INPUT);
        assert_eq!(solve_part_one(&input), 4);
    }
    
    #[test]
    fn it_solves_part_two() {
        let input = parse_bags(INPUT);
        let input_two = parse_bags(INPUT_TWO);
        assert_eq!(solve_part_two(&input), 32);
        assert_eq!(solve_part_two(&input_two), 126);
    }

}
