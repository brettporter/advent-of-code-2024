use std::cmp::Ordering;

use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

advent_of_code::solution!(5);

fn parse_input(input: &str) -> IResult<&str, (Vec<(i32, i32)>, Vec<Vec<i32>>)> {
    separated_pair(
        many1(terminated(separated_pair(i32, tag("|"), i32), opt(newline))),
        newline,
        many1(terminated(separated_list1(tag(","), i32), opt(newline))),
    )(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (rules, reprints)) = parse_input(input).unwrap();

    let mut total = 0;
    for reprint in reprints {
        let mut failed = false;
        for i in 0..reprint.len() {
            let r = reprint[i];
            for &(before, after) in &rules {
                if before == r {
                    if reprint[0..i].iter().any(|&v| v == after) {
                        failed = true;
                        break;
                    }
                }
            }
        }
        if !failed {
            total += reprint[reprint.len() / 2];
        }
    }

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, (rules, reprints)) = parse_input(input).unwrap();

    let mut total = 0;
    for mut reprint in reprints {
        let mut failed = false;
        for i in 0..reprint.len() {
            let r = reprint[i];
            for &(before, after) in &rules {
                if before == r {
                    if reprint[0..i].iter().any(|&v| v == after) {
                        failed = true;
                        break;
                    }
                }
            }
        }
        if failed {
            // TODO: reorder
            reprint.sort_by(|&a, &b| {
                let mut result = Ordering::Equal;
                for &(before, after) in &rules {
                    if before == a && after == b {
                        result = Ordering::Greater;
                        break;
                    } else if after == a && before == b {
                        result = Ordering::Less;
                        break;
                    }
                }
                result
            });
            total += reprint[reprint.len() / 2];
        }
    }

    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
