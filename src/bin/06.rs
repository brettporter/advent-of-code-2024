use nom::{
    character::complete::{newline, one_of},
    combinator::opt,
    multi::many1,
    sequence::terminated,
    IResult,
};

advent_of_code::solution!(6);

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn move_pos(&self, pos: (i32, i32)) -> (i32, i32) {
        match *self {
            Direction::UP => (pos.0, pos.1 - 1),
            Direction::DOWN => (pos.0, pos.1 + 1),
            Direction::LEFT => (pos.0 - 1, pos.1),
            Direction::RIGHT => (pos.0 + 1, pos.1),
        }
    }

    fn turn_clockwise(&self) -> Direction {
        match *self {
            Direction::UP => Self::RIGHT,
            Direction::DOWN => Self::LEFT,
            Direction::LEFT => Self::UP,
            Direction::RIGHT => Self::DOWN,
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    many1(terminated(many1(one_of(".^#")), opt(newline)))(input)
}
pub fn part_one(input: &str) -> Option<u32> {
    let (_, mut grid) = parse_input(input).unwrap();

    let size = grid.len() as i32;

    let mut pos = grid
        .iter()
        .enumerate()
        .find_map(|(row, r)| {
            if let Some(col) = r.iter().position(|&c| c == '^') {
                Some((col as i32, row as i32))
            } else {
                None
            }
        })
        .unwrap();

    grid[pos.1 as usize][pos.0 as usize] = 'X';
    let mut direction = Direction::UP;

    loop {
        let new_pos = direction.move_pos(pos);
        if new_pos.0 < 0 || new_pos.0 >= size || new_pos.1 < 0 || new_pos.1 >= size {
            break;
        }

        if grid[new_pos.1 as usize][new_pos.0 as usize] == '#' {
            direction = direction.turn_clockwise();
        } else {
            grid[new_pos.1 as usize][new_pos.0 as usize] = 'X';
            pos = new_pos;
        }
    }

    // for r in &grid {
    //     for c in r {
    //         print!("{c}");
    //     }
    //     println!();
    // }

    Some(
        grid.iter()
            .map(|r| r.iter().filter(|&&c| c == 'X').count() as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, mut grid) = parse_input(input).unwrap();

    let pos = grid
        .iter()
        .enumerate()
        .find_map(|(row, r)| {
            if let Some(col) = r.iter().position(|&c| c == '^') {
                Some((col as i32, row as i32))
            } else {
                None
            }
        })
        .unwrap();

    grid[pos.1 as usize][pos.0 as usize] = 'X';

    let mut obstructions = vec![];
    traverse_path(&grid, &Direction::UP, pos, &mut obstructions, true);
    Some(obstructions.len() as u32)
}

fn traverse_path(
    grid: &Vec<Vec<char>>,
    direction: &Direction,
    pos: (i32, i32),
    obstructions: &mut Vec<(i32, i32)>,
    place_more: bool,
) -> bool {
    let size = grid.len() as i32;
    let mut grid = grid.clone();
    let mut direction = *direction;
    let mut pos = pos;

    let mut encountered = vec![];

    loop {
        let new_pos = direction.move_pos(pos);
        if new_pos.0 < 0 || new_pos.0 >= size || new_pos.1 < 0 || new_pos.1 >= size {
            return false;
        }

        if place_more && grid[new_pos.1 as usize][new_pos.0 as usize] == '.' {
            // Try alternative
            let mut new_grid = grid.clone();
            new_grid[new_pos.1 as usize][new_pos.0 as usize] = '#';

            if traverse_path(&new_grid, &direction, pos, obstructions, false) {
                obstructions.push(new_pos);
            }
        }

        if grid[new_pos.1 as usize][new_pos.0 as usize] == '#' {
            if encountered.contains(&(new_pos, direction)) {
                return true;
            }

            encountered.push((new_pos, direction));
            direction = direction.turn_clockwise();
        } else {
            grid[new_pos.1 as usize][new_pos.0 as usize] = 'X';
            pos = new_pos;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
