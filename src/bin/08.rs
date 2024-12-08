use std::ops::RangeBounds;

use fxhash::{FxHashMap, FxHashSet};
use nom::{
    character::complete::{newline, none_of},
    combinator::opt,
    multi::many1,
    sequence::terminated,
    IResult,
};

advent_of_code::solution!(8);

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    many1(terminated(many1(none_of("\n")), opt(newline)))(input)
}

fn calculate_antinodes<R: RangeBounds<i32> + IntoIterator<Item = i32> + Clone>(
    input: &str,
    range: R,
) -> Option<u32> {
    let (_, grid) = parse_input(input).unwrap();

    let size = grid.len() as i32;
    assert_eq!(size, grid[0].len() as i32);

    let mut antennas = FxHashMap::default();

    for (row, r) in grid.iter().enumerate() {
        for (col, &c) in r.iter().enumerate() {
            if c != '.' {
                antennas
                    .entry(c)
                    .and_modify(|e: &mut Vec<(i32, i32)>| e.push((col as i32, row as i32)))
                    .or_insert(vec![(col as i32, row as i32)]);
            }
        }
    }

    let mut antinodes = FxHashSet::default();

    for (_, a) in antennas {
        for a1 in &a {
            for a2 in &a {
                if a1 != a2 {
                    // repetitively towards (and including if first step is 1) the other antenna
                    let (x_diff, y_diff) = (a2.0 - a1.0, a2.1 - a1.1);

                    for num in range.clone() {
                        let new_antinode = (a1.0 + x_diff * num, a1.1 + y_diff * num);
                        if new_antinode.0 >= 0
                            && new_antinode.0 < size
                            && new_antinode.1 >= 0
                            && new_antinode.1 < size
                        {
                            antinodes.insert(new_antinode);
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    calculate_antinodes(input, 2..=2)
}

pub fn part_two(input: &str) -> Option<u32> {
    calculate_antinodes(input, 1..)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
