use std::fmt::Error;

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use nom::{
    character::complete::{digit1, newline},
    combinator::{map_res, opt},
    multi::many1,
    sequence::terminated,
    IResult, InputIter,
};

advent_of_code::solution!(10);

enum Method {
    Score,
    Rating,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    many1(terminated(
        map_res(digit1, |s: &str| {
            Ok::<Vec<u32>, Error>(
                s.iter_elements()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect_vec(),
            )
        }),
        opt(newline),
    ))(input)
}

fn calculate_trailheads(input: &str, method: Method) -> u32 {
    let (_, trail_map) = parse_input(input).unwrap();

    let size = trail_map.len();
    assert_eq!(trail_map[0].len(), size);

    let mut trailheads = vec![];
    let mut connections = FxHashMap::default();

    for (row, r) in trail_map.iter().enumerate() {
        for (col, &c) in r.iter().enumerate() {
            let entry = connections.entry((col, row)).or_insert(vec![]);
            if row > 0 && trail_map[row - 1][col] == c + 1 {
                entry.push((col, row - 1));
            }
            if row < size - 1 && trail_map[row + 1][col] == c + 1 {
                entry.push((col, row + 1));
            }
            if col > 0 && trail_map[row][col - 1] == c + 1 {
                entry.push((col - 1, row));
            }
            if col < size - 1 && trail_map[row][col + 1] == c + 1 {
                entry.push((col + 1, row));
            }

            if c == 0 {
                trailheads.push((col, row));
            }
        }
    }

    trailheads
        .iter()
        .map(|trailhead| {
            let mut found = FxHashSet::default();
            let mut count = 0;
            let mut remaining = vec![trailhead];
            while !remaining.is_empty() {
                let next = remaining.pop().unwrap();
                if trail_map[next.1][next.0] == 9 {
                    found.insert(next);
                    count += 1;
                } else {
                    remaining.extend(connections[next].iter());
                }
            }
            match method {
                Method::Score => found.len() as u32,
                Method::Rating => count,
            }
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(calculate_trailheads(input, Method::Score))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(calculate_trailheads(input, Method::Rating))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
