use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use nom::{
    character::complete::{newline, one_of},
    combinator::opt,
    multi::many1,
    sequence::terminated,
    IResult,
};
use priority_queue::DoublePriorityQueue;

advent_of_code::solution!(16);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Path {
    direction: Direction,
    pos: (usize, usize),
}

impl Path {
    fn get_scored_directions(&self) -> [(Path, u32); 3] {
        let pos = self.pos;
        match self.direction {
            Direction::North => [
                (
                    Path {
                        direction: Direction::North,
                        pos: (pos.0, pos.1 - 1),
                    },
                    1,
                ),
                (
                    Path {
                        direction: Direction::East,
                        pos: (pos.0 + 1, pos.1),
                    },
                    1001,
                ),
                (
                    Path {
                        direction: Direction::West,
                        pos: (pos.0 - 1, pos.1),
                    },
                    1001,
                ),
            ],
            Direction::South => [
                (
                    Path {
                        direction: Direction::South,
                        pos: (pos.0, pos.1 + 1),
                    },
                    1,
                ),
                (
                    Path {
                        direction: Direction::East,
                        pos: (pos.0 + 1, pos.1),
                    },
                    1001,
                ),
                (
                    Path {
                        direction: Direction::West,
                        pos: (pos.0 - 1, pos.1),
                    },
                    1001,
                ),
            ],
            Direction::East => [
                (
                    Path {
                        direction: Direction::East,
                        pos: (pos.0 + 1, pos.1),
                    },
                    1,
                ),
                (
                    Path {
                        direction: Direction::North,
                        pos: (pos.0, pos.1 - 1),
                    },
                    1001,
                ),
                (
                    Path {
                        direction: Direction::South,
                        pos: (pos.0, pos.1 + 1),
                    },
                    1001,
                ),
            ],
            Direction::West => [
                (
                    Path {
                        direction: Direction::West,
                        pos: (pos.0 - 1, pos.1),
                    },
                    1,
                ),
                (
                    Path {
                        direction: Direction::North,
                        pos: (pos.0, pos.1 - 1),
                    },
                    1001,
                ),
                (
                    Path {
                        direction: Direction::South,
                        pos: (pos.0, pos.1 + 1),
                    },
                    1001,
                ),
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

    let start = Path {
        pos: find_item(&grid, 'S'),
        direction: Direction::East,
    };

    let mut queue = DoublePriorityQueue::new();
    queue.push(start, 0);

    let mut cost_tally = FxHashMap::default();
    cost_tally.insert(start, 0);

    while let Some((cur_path, cost)) = queue.pop_min() {
        if grid[cur_path.pos.1][cur_path.pos.0] == 'E' {
            return Some(cost);
        }

        let options = cur_path.get_scored_directions();
        let valid_options = options
            .iter()
            .filter(|(path, _)| grid[path.pos.1][path.pos.0] != '#')
            .collect_vec();

        for &(n, score_inc) in valid_options {
            let score = cost + score_inc;
            if score < *cost_tally.get(&n).unwrap_or(&u32::MAX) {
                cost_tally.insert(n, score);
                queue.push(n, score);
            }
        }
    }

    None
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
