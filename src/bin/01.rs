use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::opt,
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let (_, values) = parse_input(input).unwrap();

    let mut list1 = values.iter().map(|v| v.0).collect_vec();
    let mut list2 = values.iter().map(|v| v.1).collect_vec();
    list1.sort();
    list2.sort();

    let mut total = 0;
    for i in 0..list1.len() {
        total += (list1[i] - list2[i]).abs();
    }

    Some(total)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    many1(terminated(
        separated_pair(i32, tag("   "), i32),
        opt(newline),
    ))(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, values) = parse_input(input).unwrap();

    let mut total = 0;

    let list1 = values.iter().map(|v| v.0).collect_vec();
    let list2 = values.iter().map(|v| v.1).collect_vec();

    for v in list1 {
        total += list2.iter().filter(|&x| *x == v).count() as u32 * v as u32;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
