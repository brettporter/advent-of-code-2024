use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::i32,
    multi::many0,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};
use regex::Regex;

advent_of_code::solution!(3);

fn parse_input(input: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| (c[1].parse().unwrap(), c[2].parse().unwrap()))
        .collect_vec()
}

// fn parse_input(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
// This won't work for mul that doesn't match
// many0(preceded(
//     take_until("mul"),
//     preceded(
//         tag("mul"),
//         delimited(tag("("), separated_pair(i32, tag(","), i32), tag(")")),
//     ),
// ))(input)
// }

pub fn part_one(input: &str) -> Option<u32> {
    // let (_, arguments) = parse_input(input).unwrap();
    let arguments = parse_input(input);

    Some(arguments.iter().map(|&(v1, v2)| v1 * v2).sum())
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
