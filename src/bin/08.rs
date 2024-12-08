use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
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

pub fn part_one(input: &str) -> Option<u32> {
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
        for pair in a.iter().combinations(2) {
            assert_eq!(pair.len(), 2);

            let (&a1, &a2) = (pair[0], pair[1]);

            let x_diff = a1.0 - a2.0;
            let y_diff = a1.1 - a2.1;

            for new_antinode in vec![
                (a1.0 + x_diff, a1.1 + y_diff),
                (a2.0 - x_diff, a2.1 - y_diff),
            ] {
                if new_antinode.0 >= 0
                    && new_antinode.0 < size
                    && new_antinode.1 >= 0
                    && new_antinode.1 < size
                {
                    antinodes.insert(new_antinode);
                }
            }
        }
    }

    Some(antinodes.len() as u32)
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
