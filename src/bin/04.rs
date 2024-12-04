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
    let grid = read_padded_grid(input, 3);

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
                        let x = (col + direction.0 * idx) as usize;
                        let y = (row + direction.1 * idx) as usize;

                        grid[y][x] == WORD[idx as usize]
                    }) {
                        count += 1;
                    }
                }
            }
        }
    }

    Some(count)
}

fn read_padded_grid(input: &str, padding: usize) -> Vec<Vec<char>> {
    let (_, grid) = parse_input(input).unwrap();

    let pad_rows = vec![vec![' '; grid[0].len() + padding * 2]; padding];

    let mut result = pad_rows.clone();
    result.extend(grid.iter().map(|row| {
        let mut padded_row = vec![' '; padding];
        padded_row.extend(row.chars());
        padded_row.extend(vec![' '; padding]);
        padded_row
    }));
    result.extend(pad_rows);
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = read_padded_grid(input, 1);

    let size = grid.len() as i32;

    const DIRECTIONS: [(i32, i32); 4] = [(-1, -1), (1, -1), (-1, 1), (1, 1)];
    let mut count = 0;
    for row in 0..size {
        assert_eq!(size as usize, grid[row as usize].len());
        for col in 0..size {
            if grid[row as usize][col as usize] == 'A' {
                if DIRECTIONS.iter().all(|(c, r)| {
                    let x = (col + c) as usize;
                    let y = (row + r) as usize;
                    let opp_x = (col - c) as usize;
                    let opp_y = (row - r) as usize;

                    (grid[y as usize][x] == 'M' || grid[y][x] == 'S')
                        && (grid[opp_y][opp_x] != grid[y][x])
                }) {
                    count += 1;
                }
            }
        }
    }

    Some(count)
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
        assert_eq!(result, Some(9));
    }
}
