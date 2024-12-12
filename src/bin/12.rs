use fxhash::FxHashMap;
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
}

impl Region {
    fn new(boundaries: u32) -> Self {
        Self {
            area: 1,
            perimeter: boundaries,
        }
    }

    fn update(&mut self, boundaries: u32) {
        self.area += 1;
        self.perimeter += boundaries;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, grid) = parse_input(input).unwrap();
    let size = grid.len();
    assert_eq!(size, grid[0].len());

    todo!("disjoint sets");
    let mut regions = FxHashMap::default();

    for (row, r) in grid.iter().enumerate() {
        for (col, &c) in r.iter().enumerate() {
            let mut boundaries = 0;

            if row == 0 || grid[row - 1][col] != c {
                boundaries += 1;
            }
            if row == size - 1 || grid[row + 1][col] != c {
                boundaries += 1;
            }
            if col == 0 || grid[row][col - 1] != c {
                boundaries += 1;
            }
            if col == size - 1 || grid[row][col + 1] != c {
                boundaries += 1;
            }

            println!("{} {:?} {}", c, (col, row), boundaries);

            regions
                .entry(c)
                .and_modify(|r: &mut Region| r.update(boundaries))
                .or_insert(Region::new(boundaries));
        }
    }

    println!("{:#?}", regions);

    Some(regions.iter().map(|(_, v)| v.perimeter * v.area).sum())
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
