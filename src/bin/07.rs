use nom::{
    bytes::complete::tag,
    character::complete::{newline, space1, u64},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

advent_of_code::solution!(7);

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Mult,
    Concat,
}

impl Operator {
    fn apply(&self, v1: u64, v2: u64) -> u64 {
        match *self {
            Self::Add => v1 + v2,
            Self::Mult => v1 * v2,
            Self::Concat => format!("{}{}", v1, v2).parse().unwrap(),
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    many1(terminated(
        separated_pair(u64, tag(": "), separated_list1(space1, u64)),
        opt(newline),
    ))(input)
}

fn find_result(v: u64, remaining: &[u64], operators: &[Operator], result: &u64) -> bool {
    if remaining.is_empty() {
        v == *result
    } else {
        operators.iter().any(|op| {
            find_result(
                op.apply(v, remaining[0]),
                &remaining[1..],
                operators,
                result,
            )
        })
    }
}

fn count_equations(eqs: &Vec<(u64, Vec<u64>)>, operators: &[Operator]) -> u64 {
    eqs.iter()
        .filter_map(|(result, args)| {
            if operators
                .iter()
                .any(|op| find_result(op.apply(args[0], args[1]), &args[2..], operators, result))
            {
                Some(result)
            } else {
                None
            }
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, eqs) = parse_input(input).unwrap();

    Some(count_equations(&eqs, &[Operator::Add, Operator::Mult]))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, eqs) = parse_input(input).unwrap();

    Some(count_equations(
        &eqs,
        &[Operator::Add, Operator::Mult, Operator::Concat],
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
