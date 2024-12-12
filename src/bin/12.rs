use fxhash::FxHashSet;
use nom::{
    character::complete::{newline, none_of},
    combinator::opt,
    multi::many1,
    sequence::terminated,
    IResult,
};

advent_of_code::solution!(12);

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    many1(terminated(many1(none_of("\n")), opt(newline)))(input)
}

#[derive(Debug)]
struct Region {
    area: u32,
    perimeter: u32,
    sides: u32,
}

impl Region {
    fn from(
        grid: &Vec<Vec<char>>,
        col: usize,
        row: usize,
        visited: &mut FxHashSet<(usize, usize)>,
    ) -> Self {
        let (area, perimeter, corners) = traverse(grid, col, row, visited);

        Self {
            area,
            perimeter,
            sides: (corners - 2) * 2,
        }
    }
}

fn traverse(
    grid: &Vec<Vec<char>>,
    col: usize,
    row: usize,
    visited: &mut FxHashSet<(usize, usize)>,
) -> (u32, u32, u32) {
    if visited.contains(&(col, row)) {
        return (0, 0, 0);
    }
    let size = grid.len();
    assert_eq!(size, grid[0].len());

    let mut area = 1;
    let mut perimeter = 0;
    let mut corners = 0;
    let c = grid[row][col];
    visited.insert((col, row));

    let (mut up_edge, mut down_edge, mut left_edge, mut right_edge) = (false, false, false, false);

    if row == 0 || grid[row - 1][col] != c {
        perimeter += 1;
        up_edge = true;
    } else {
        let up = traverse(grid, col, row - 1, visited);
        area += up.0;
        perimeter += up.1;
        corners += up.2;
    }
    if row == size - 1 || grid[row + 1][col] != c {
        perimeter += 1;
        down_edge = true;
    } else {
        let down = traverse(grid, col, row + 1, visited);
        area += down.0;
        perimeter += down.1;
        corners += down.2;
    }
    if col == 0 || grid[row][col - 1] != c {
        perimeter += 1;
        left_edge = true;
    } else {
        let left = traverse(grid, col - 1, row, visited);
        area += left.0;
        perimeter += left.1;
        corners += left.2;
    }
    if col == size - 1 || grid[row][col + 1] != c {
        perimeter += 1;
        right_edge = true;
    } else {
        let right = traverse(grid, col + 1, row, visited);
        area += right.0;
        perimeter += right.1;
        corners += right.2;
    }

    if left_edge && down_edge {
        corners += 1;
    }
    if left_edge && up_edge {
        corners += 1;
    }
    if right_edge && down_edge {
        corners += 1;
    }
    if right_edge && up_edge {
        corners += 1;
    }

    // TODO: instead of corners, let's try this:
    //  * gather sides in arrays by vert (up, down) and horiz (left, right)
    //  * sort the array for each vert coordinate and each horiz coordinate
    //  * count all for perimeter
    //  * collapse sequential items to count sides
    (area, perimeter, corners)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, grid) = parse_input(input).unwrap();

    let mut regions = vec![];
    let mut visited = FxHashSet::default();

    for (row, r) in grid.clone().iter().enumerate() {
        for (col, _) in r.iter().enumerate() {
            if !visited.contains(&(col, row)) {
                regions.push(Region::from(&grid, col, row, &mut visited));
            }
        }
    }

    Some(regions.iter().map(|v| v.perimeter * v.area).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, grid) = parse_input(input).unwrap();

    let mut regions = vec![];
    let mut visited = FxHashSet::default();

    for (row, r) in grid.clone().iter().enumerate() {
        for (col, c) in r.iter().enumerate() {
            if !visited.contains(&(col, row)) {
                let region = Region::from(&grid, col, row, &mut visited);
                regions.push(region);
            }
        }
    }

    Some(regions.iter().map(|v| v.sides * v.area).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 21,
        ));
        assert_eq!(result, Some(80));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 22,
        ));
        assert_eq!(result, Some(436));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 23,
        ));
        assert_eq!(result, Some(236));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 24,
        ));
        assert_eq!(result, Some(368));
    }
}
