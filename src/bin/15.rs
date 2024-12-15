use std::io::Empty;

use itertools::Itertools;
use nom::{
    character::complete::{newline, one_of},
    combinator::opt,
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

advent_of_code::solution!(15);

#[derive(Debug, PartialEq, Copy, Clone)]
enum Entity {
    Wall,
    Box,
    Robot,
    Empty,
}

impl Entity {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            'O' => Self::Box,
            '.' => Self::Empty,
            '@' => Self::Robot,
            _ => unreachable!("Invalid input"),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from(c: char) -> Self {
        match c {
            'v' => Self::Down,
            '^' => Self::Up,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unreachable!("Inavlid input"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}
impl Coord {
    fn next_position(&self, direction: &Direction) -> Coord {
        match direction {
            Direction::Up => Coord {
                y: self.y - 1,
                x: self.x,
            },
            Direction::Down => Coord {
                y: self.y + 1,
                x: self.x,
            },
            Direction::Left => Coord {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Coord {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Vec<Entity>>, Vec<Direction>)> {
    fn parse_map(input: &str) -> IResult<&str, Vec<Vec<Entity>>> {
        many1(terminated(
            many1(one_of("#.O@").map(|c| Entity::from(c))),
            newline,
        ))(input)
    }
    fn parse_moves(input: &str) -> IResult<&str, Vec<Direction>> {
        many1(terminated(
            one_of("^v<>").map(|c| Direction::from(c)),
            opt(newline),
        ))(input)
    }
    separated_pair(parse_map, newline, parse_moves)(input)
}

fn move_entity(pos: &Coord, direction: Direction, grid: &mut [Vec<Entity>]) -> Coord {
    let next_pos = pos.next_position(&direction);

    if grid[next_pos.y][next_pos.x] == Entity::Box {
        move_entity(&next_pos, direction, grid);
    }

    if grid[next_pos.y][next_pos.x] == Entity::Empty {
        grid[next_pos.y][next_pos.x] = grid[pos.y][pos.x];
        grid[pos.y][pos.x] = Entity::Empty;
        next_pos
    } else {
        *pos
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (mut grid, moves)) = parse_input(input).unwrap();

    let mut robot_pos = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            if let Some(x) = row.iter().position(|&entity| entity == Entity::Robot) {
                Some(Coord { x, y })
            } else {
                None
            }
        })
        .unwrap();

    for m in moves {
        robot_pos = move_entity(&robot_pos, m, &mut grid);
    }

    let mut total = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &entity) in row.iter().enumerate() {
            if entity == Entity::Box {
                total += y * 100 + x;
            }
        }
    }

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 11,
        ));
        assert_eq!(result, Some(2028));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 12,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
