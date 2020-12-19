#[derive(Debug)]
enum Token {
    Number(isize),
    Operator(Operator),
    LeftParent,
    RightParent
}

#[derive(Debug)]
enum Operator {
    Add,
    Mul
}

fn parse_tokens(input: &str) -> Vec<Token> {
    input.split_whitespace()
        .flat_map(|part| match part {
            "*" => vec![Token::Operator(Operator::Mul)],
            "+" => vec![Token::Operator(Operator::Add)],
            part => {
                let mut res = Vec::new();
                
                part.chars()
                    .filter(|x| x == &'(')
                    .for_each(|_| res.push(Token::LeftParent));

                let digits = part.chars()
                    .filter(|x| x.is_numeric())
                    .map(|x| x.to_digit(10).unwrap())
                    .fold(0, |sum, x| (sum * 10) + x as isize);
                res.push(Token::Number(digits));
            
                part.chars()
                    .filter(|x| x == &')')
                    .for_each(|_| res.push(Token::RightParent));

                res
            }
        })
        .collect()
}

fn evaluate_equal_precedence(input: &str) -> isize {
    parse_tokens(input)
        .into_iter()
        .fold((0, Vec::new(), None), |(prev_result, mut stack, prev_operator), token| {
            match token {
                Token::Number(value) => match prev_operator {
                    Some(Operator::Add) => (prev_result + value, stack, None),
                    Some(Operator::Mul) => (prev_result * value, stack, None),
                    None => (value, stack, prev_operator)
                },
                Token::Operator(operator) => (prev_result, stack, Some(operator)),
                Token::LeftParent => {
                    stack.push((prev_result, prev_operator));
                    (prev_result, stack, None)
                }
                Token::RightParent => {
                    let (result, operator) = stack.pop().unwrap();
                    match operator {
                        Some(Operator::Add) => (result + prev_result, stack, None),
                        Some(Operator::Mul) => (result * prev_result, stack, None),
                        None => (prev_result, stack, prev_operator)
                    }
                }
            }
        })
        .0
}

fn evaluate_different_precedence(input: &str) -> isize {
    let (result, multiplication, _, _ ) = parse_tokens(input)
        .into_iter()
        .fold((0, 1, Vec::new(), None), |(prev_result, prev_multiplication, mut stack, prev_operator), token| {
            match token {
                Token::Number(value) => match prev_operator {
                    Some(Operator::Add) => (prev_result + value, prev_multiplication, stack, None),
                    Some(Operator::Mul) => (value, prev_result * prev_multiplication, stack, None),
                    None => (value, prev_multiplication, stack, prev_operator)
                },
                Token::Operator(operator) => (prev_result, prev_multiplication, stack, Some(operator)),
                Token::LeftParent => {
                    stack.push((prev_result, prev_multiplication, prev_operator));
                    (prev_result, 1, stack, None)
                }
                Token::RightParent => {
                    let prev_result = prev_result * prev_multiplication;
                    let (result, multiplication, operator) = stack.pop().unwrap();
                    match operator {
                        Some(Operator::Add) => (result + prev_result, multiplication, stack, None),
                        Some(Operator::Mul) => (prev_result, multiplication * result, stack, None),
                        None => (prev_result, multiplication, stack, prev_operator)
                    }
                }
            }
        });
        result * multiplication
}

#[aoc(day18, part1)]
pub fn solve_part_one(input: &str) -> isize {
    input.lines()
        .map(evaluate_equal_precedence)
        .sum()
}

#[aoc(day18, part2)]
pub fn solve_part_two(input: &str) -> isize {
    input.lines()
        .map(evaluate_different_precedence)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_solves_part_one() {
        assert_eq!(solve_part_one("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(solve_part_one("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(solve_part_one("2 * 3 + (4 * 5)"), 26);
        assert_eq!(solve_part_one("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(solve_part_one("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(solve_part_one("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
    }
    
    #[test]
    fn it_solves_part_two() {
        assert_eq!(solve_part_two("1 + 2 * 3 + 4 * 5 + 6"), 231);
        assert_eq!(solve_part_one("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(solve_part_two("2 * 3 + (4 * 5)"), 46);
        assert_eq!(solve_part_two("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(solve_part_two("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
        assert_eq!(solve_part_two("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);
    }

}
