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
}

impl Operator {
    fn apply(&self, v1: u64, v2: u64) -> u64 {
        match *self {
            Self::Add => v1 + v2,
            Self::Mult => v1 * v2,
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    many1(terminated(
        separated_pair(u64, tag(": "), separated_list1(space1, u64)),
        opt(newline),
    ))(input)
}

fn find_result(a: u64, op: Operator, remaining: &[u64], result: u64) -> bool {
    if remaining.is_empty() {
        a == result
    } else {
        find_result(
            op.apply(a, remaining[0]),
            Operator::Add,
            &remaining[1..],
            result,
        ) || find_result(
            op.apply(a, remaining[0]),
            Operator::Mult,
            &remaining[1..],
            result,
        )
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, eqs) = parse_input(input).unwrap();

    let mut total = 0;

    for (result, args) in eqs {
        if find_result(args[0], Operator::Add, &args[1..], result)
            || find_result(args[0], Operator::Mult, &args[1..], result)
        {
            total += result;
        }
    }

    Some(total as u64)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
