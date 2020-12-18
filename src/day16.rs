use std::ops::Range;

pub struct Rule {
    field: String,
    ranges: Vec<Range<usize>>
}

type Ticket = Vec<usize>;

pub struct Input {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>
}

fn parse_range(input: &str) -> Range<usize> {
    let parts = input
        .split('-')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();
    Range { start: parts[0], end: parts[1] + 1}
}

fn parse_rule(input: &str) -> Rule {
    let parts = input.split(':').collect::<Vec<_>>();
    let field = parts[0].to_string();
    let parts = parts[1].split_whitespace().collect::<Vec<_>>();
    let first_range = parse_range(parts[0]);
    let second_range = parse_range(parts[2]);
    Rule { field, ranges: vec![first_range, second_range] }
}

fn parse_ticket(input: &str) -> Vec<usize> {
    input.split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn rule_matches_value(rule: &Rule, value: &usize) -> bool {
    rule.ranges.iter().any(|range| range.contains(value))
}

fn find_invalid_ticket_values(rules: &[Rule], ticket: &Ticket) -> Vec<usize> {
    ticket.iter()
        .filter(|field| !rules.iter().any(|rule| rule_matches_value(rule, field)))
        .map(|x| *x)
        .collect()
}

fn is_ticket_valid(rules: &[Rule], ticket: &Ticket) -> bool {
    find_invalid_ticket_values(rules, ticket).len() == 0
}

fn find_all_valid_fields(rules: &[Rule], tickets: &[Ticket]) -> Vec<Vec<String>> {
    let valid_tickets = tickets.iter()
        .filter(|ticket| is_ticket_valid(rules, ticket))
        .collect::<Vec<_>>();
    (0..rules.len())
        .map(|index| valid_tickets.iter().map(|ticket| ticket[index]).collect::<Vec<_>>())
        .map(|values| {
            rules
                .iter()
                .rev()
                .filter(|rule| values.iter().all(|value| rule_matches_value(rule, value)))
                .map(|rule| rule.field.to_string())
                .collect()
        })
        .collect()
}

fn calculate_permutation(options: &[Vec<String>]) -> Vec<String> {
    let mut fields: Vec<Option<&str>> = vec![None; options.len()];
    let mut options = options.iter()
        .map(|x| x.iter().map(|x| x.as_str()).collect())    
        .collect::<Vec<Vec<&str>>>();
    loop {
        if fields.iter().filter(|field| field.is_none()).count() == 0 { break }
        let (index, option) = options.iter()
            .enumerate()
            .find(|(_, vals)| vals.len() == 1)
            .unwrap();
        fields[index] = Some(option[0]);
        options = options.iter()
            .map(|vals| {
                vals.iter()
                    .filter(|&val| val != &option[0])
                    .map(|&x| x)
                    .collect()
            })
            .collect::<Vec<_>>();
    }
    fields
        .iter()
        .filter_map(|x| *x)
        .map(|x| x.to_owned())
        .collect()
}

fn find_fields(rules: &[Rule], tickets: &[Ticket]) -> Vec<String> {
    let options = find_all_valid_fields(rules, tickets);
    calculate_permutation(&options)
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Input {
    let mut rules = Vec::new();
    let mut lines = input.lines();
    
    while let Some(line) = lines.next() {
        if line.trim().is_empty() { break; }
        rules.push(parse_rule(line));
    }

    lines.next();
    let my_ticket = parse_ticket(lines.next().unwrap());

    lines.next();
    lines.next();
    let mut nearby_tickets = Vec::new();
    while let Some(line) = lines.next() {
        nearby_tickets.push(parse_ticket(line));
    }

    Input { rules, my_ticket, nearby_tickets }
}

#[aoc(day16, part1)]
pub fn solve_part_one(input: &Input) -> usize {
    input.nearby_tickets.iter()
        .flat_map(|ticket| find_invalid_ticket_values(&input.rules, ticket))
        .sum()
}

#[aoc(day16, part2)]
pub fn solve_part_two(input: &Input) -> usize {
    find_fields(&input.rules, &input.nearby_tickets)
        .iter()
        .enumerate()
        .filter_map(|(index, field)| {
            if field.starts_with("departure") {
                Some(input.my_ticket[index])
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT_ONE: &str = indoc! {"
        class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50

        your ticket:
        7,1,14

        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12
    "};

    #[test]
    fn it_solves_part_one() {
        let input = input_generator(INPUT_ONE);
        assert_eq!(solve_part_one(&input), 71);
    }

    const INPUT_TWO: &str = indoc! {"
        class: 0-1 or 4-19
        row: 0-5 or 8-19
        seat: 0-13 or 16-19
        
        your ticket:
        11,12,13
        
        nearby tickets:
        3,9,18
        15,1,5
        5,14,9
    "};
    
    #[test]
    fn it_solves_part_two() {
        let input = input_generator(INPUT_TWO);
        assert_eq!(find_fields(&input.rules, &input.nearby_tickets), vec!["row".to_string(), "class".to_string(), "seat".to_string()]);
        // assert_eq!(solve_part_two(&input), 12 * 13);
    }

}
