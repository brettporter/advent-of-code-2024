use itertools::Itertools;
use nom::{
    character::complete::{alpha1, newline},
    combinator::opt,
    multi::many1,
    sequence::terminated,
    IResult,
};

advent_of_code::solution!(4);

fn parse_input(input: &str) -> IResult<&str, Vec<&str>> {
    many1(terminated(alpha1, opt(newline)))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, grid) = parse_input(input).unwrap();

    let grid = grid
        .iter()
        .map(|row| row.chars().collect_vec())
        .collect_vec();

    let size = grid.len() as i32;

    const WORD: [char; 4] = ['X', 'M', 'A', 'S'];
    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let mut count = 0;
    for row in 0..size {
        assert_eq!(size as usize, grid[row as usize].len());
        for col in 0..size {
            if grid[row as usize][col as usize] == 'X' {
                for direction in DIRECTIONS {
                    if (1..=3).all(|idx| {
                        let x = col + direction.0 * idx;
                        let y = row + direction.1 * idx;

                        x >= 0
                            && x < size
                            && y >= 0
                            && y < size
                            && grid[y as usize][x as usize] == WORD[idx as usize]
                    }) {
                        count += 1;
                    }
                }
            }
        }
    }

    Some(count)
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}