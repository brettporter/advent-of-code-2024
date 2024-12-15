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
    BoxL,
    BoxR,
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

    fn to_char(&self) -> char {
        match *self {
            Self::Wall => '#',
            Self::Box => 'O',
            Self::BoxL => '[',
            Self::BoxR => ']',
            Self::Empty => '.',
            Self::Robot => '@',
        }
    }

    fn expand(&self) -> [Entity; 2] {
        match *self {
            Entity::Box => [Entity::BoxL, Entity::BoxR],
            Entity::Empty => [Entity::Empty, Entity::Empty],
            Entity::Robot => [Entity::Robot, Entity::Empty],
            Entity::Wall => [Entity::Wall, Entity::Wall],
            _ => unreachable!("Already expanded"),
        }
    }

    fn is_box(&self) -> bool {
        *self == Entity::Box || *self == Entity::BoxL || *self == Entity::BoxR
    }
}

#[derive(Debug, PartialEq)]
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

    fn is_vertical(&self) -> bool {
        *self == Self::Up || *self == Self::Down
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

fn move_entity(
    pos: &Coord,
    direction: &Direction,
    grid: &mut [Vec<Entity>],
    is_other_half: bool,
) -> Option<Coord> {
    let next_pos = pos.next_position(&direction);

    println!(
        "Try moving {:?} @ {:?} {:?} -> {:?}",
        grid[pos.y][pos.x], pos, direction, next_pos
    );

    if grid[next_pos.y][next_pos.x].is_box() {
        move_entity(&next_pos, direction, grid, false);
    }

    if grid[next_pos.y][next_pos.x] == Entity::Empty {
        let e = grid[pos.y][pos.x];
        if e == Entity::BoxL && direction.is_vertical() && !is_other_half {
            let other_half = pos.next_position(&Direction::Right);
            if move_entity(&other_half, direction, grid, true).is_none() {
                return None;
            }
        } else if e == Entity::BoxR && direction.is_vertical() && !is_other_half {
            let other_half = pos.next_position(&Direction::Left);
            if move_entity(&other_half, direction, grid, true).is_none() {
                return None;
            }
        }

        grid[next_pos.y][next_pos.x] = e;
        grid[pos.y][pos.x] = Entity::Empty;
        Some(next_pos)
    } else {
        None
    }
}

fn find_robot(grid: &Vec<Vec<Entity>>) -> Coord {
    grid.iter()
        .enumerate()
        .find_map(|(y, row)| {
            if let Some(x) = row.iter().position(|&entity| entity == Entity::Robot) {
                Some(Coord { x, y })
            } else {
                None
            }
        })
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (mut grid, moves)) = parse_input(input).unwrap();

    let mut robot_pos = find_robot(&grid);

    for m in moves {
        if let Some(p) = move_entity(&robot_pos, &m, &mut grid, false) {
            robot_pos = p;
        }
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
    let (_, (grid, moves)) = parse_input(input).unwrap();

    // TODO: needed?
    let size = grid.len();
    assert_eq!(grid[0].len(), size);

    let mut grid = grid
        .iter()
        .map(|row| row.iter().flat_map(|entity| entity.expand()).collect())
        .collect_vec();

    let mut robot_pos = find_robot(&grid);

    for m in moves {
        if let Some(p) = move_entity(&robot_pos, &m, &mut grid, false) {
            robot_pos = p;
        }
    }

    for row in &grid {
        for col in row {
            print!("{}", col.to_char())
        }
        println!();
    }

    let mut total = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &entity) in row.iter().enumerate() {
            if entity == Entity::BoxL {
                total += y * 100 + x;
            }
        }
    }

    Some(total as u32)
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

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
