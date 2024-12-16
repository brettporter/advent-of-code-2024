use std::{collections::VecDeque, u32};

use fxhash::FxHashSet;
use itertools::Itertools;
use nom::{
    character::complete::{newline, one_of},
    combinator::opt,
    multi::many1,
    sequence::terminated,
    IResult,
};

advent_of_code::solution!(16);

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, Debug)]
struct Path {
    score: u32,
    direction: Direction,
    pos: (usize, usize),
}

impl Path {
    fn get_scored_directions(&self) -> [Path; 3] {
        let pos = self.pos;
        let score = self.score;
        match self.direction {
            Direction::North => [
                Path {
                    direction: Direction::North,
                    score: score + 1,
                    pos: (pos.0, pos.1 - 1),
                },
                Path {
                    direction: Direction::East,
                    score: score + 1001,
                    pos: (pos.0 + 1, pos.1),
                },
                Path {
                    direction: Direction::West,
                    score: score + 1001,
                    pos: (pos.0 - 1, pos.1),
                },
            ],
            Direction::South => [
                Path {
                    direction: Direction::South,
                    score: score + 1,
                    pos: (pos.0, pos.1 + 1),
                },
                Path {
                    direction: Direction::East,
                    score: score + 1001,
                    pos: (pos.0 + 1, pos.1),
                },
                Path {
                    direction: Direction::West,
                    score: score + 1001,
                    pos: (pos.0 - 1, pos.1),
                },
            ],
            Direction::East => [
                Path {
                    direction: Direction::East,
                    score: score + 1,
                    pos: (pos.0 + 1, pos.1),
                },
                Path {
                    direction: Direction::North,
                    score: score + 1001,
                    pos: (pos.0, pos.1 - 1),
                },
                Path {
                    direction: Direction::South,
                    score: score + 1001,
                    pos: (pos.0, pos.1 + 1),
                },
            ],
            Direction::West => [
                Path {
                    direction: Direction::West,
                    score: score + 1,
                    pos: (pos.0 - 1, pos.1),
                },
                Path {
                    direction: Direction::North,
                    score: score + 1001,
                    pos: (pos.0, pos.1 - 1),
                },
                Path {
                    direction: Direction::South,
                    score: score + 1001,
                    pos: (pos.0, pos.1 + 1),
                },
            ],
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    many1(terminated(many1(one_of("#.SE")), opt(newline)))(input)
}

fn find_item(grid: &Vec<Vec<char>>, item: char) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .find_map(|(y, row)| {
            if let Some(x) = row.iter().position(|&c| c == item) {
                Some((x, y))
            } else {
                None
            }
        })
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, grid) = parse_input(input).unwrap();

    let start = find_item(&grid, 'S');
    let mut best = None;
    let mut paths = VecDeque::new();
    let mut visited = FxHashSet::default();
    visited.insert(start);

    paths.push_front((
        Path {
            score: 0,
            direction: Direction::East,
            pos: start,
        },
        visited,
    ));

    while let Some((p, mut visited)) = paths.pop_front() {
        let mut cur_path = p;
        println!("Trying {:?}", cur_path);

        // No loop detection yet, no backtracking
        loop {
            if best.is_some() && cur_path.score > best.unwrap() {
                println!("Abandon - score too high {:?}", cur_path);
                break;
            }
            visited.insert(cur_path.pos);

            let options = cur_path.get_scored_directions();

            let valid_options = options
                .iter()
                .filter(|path| grid[path.pos.1][path.pos.0] != '#' && !visited.contains(&path.pos))
                .collect_vec();

            if valid_options.len() == 0 {
                println!(
                    "Abandon - blocked at {:?}, options were {:?}",
                    cur_path, options
                );
                break;
            }

            for &o in &valid_options[1..] {
                paths.push_back((*o, visited.clone()));
            }

            cur_path = **valid_options.first().unwrap();
            if grid[cur_path.pos.1][cur_path.pos.0] == 'E' {
                println!("Found end {:?}", cur_path);
                if let Some(b) = best {
                    best = Some(b.min(cur_path.score));
                } else {
                    best = Some(cur_path.score);
                }
                break;
            }
            println!("Moving to {:?}", cur_path);
        }
    }

    best
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
        assert_eq!(result, Some(7036));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 12,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
