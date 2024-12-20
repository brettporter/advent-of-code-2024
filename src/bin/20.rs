use fxhash::FxHashMap;
use itertools::Itertools;
use nom::{
    character::complete::{newline, one_of},
    combinator::opt,
    multi::many1,
    sequence::terminated,
    IResult,
};

advent_of_code::solution!(20);

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    many1(terminated(many1(one_of("#.SE")), opt(newline)))(input)
}

fn find_item(grid: &Vec<Vec<char>>, item: char) -> (i32, i32) {
    grid.iter()
        .enumerate()
        .find_map(|(y, row)| {
            if let Some(x) = row.iter().position(|&c| c == item) {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .unwrap()
}

fn part_one_with_limit(input: &str, limit: i32) -> Option<u32> {
    let (_, grid) = parse_input(input).unwrap();

    let size = grid.len() as i32;
    assert_eq!(grid[0].len() as i32, size);
    let start = find_item(&grid, 'S');

    // Navigate S -> E and save the distance from start, will use this repeat with cheats

    let mut pos = start;
    let mut last = start;
    let mut path_nodes = FxHashMap::from_iter([(start, 0)]);
    let mut path = vec![start];

    while grid[pos.1 as usize][pos.0 as usize] != 'E' {
        // TODO: use find instead
        let options = [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .iter()
            .filter(|&&p| {
                let next_pos = (pos.0 + p.0, pos.1 + p.1);
                next_pos != last
                    && next_pos.0 >= 0
                    && next_pos.0 < size
                    && next_pos.1 >= 0
                    && next_pos.1 < size
                    && grid[next_pos.1 as usize][next_pos.0 as usize] != '#'
            })
            .collect_vec();

        assert!(options.len() == 1);
        last = pos;
        let (dx, dy) = options.first().unwrap();
        pos = (pos.0 + dx, pos.1 + dy);
        path_nodes.insert(pos, path_nodes.len() as i32);
        path.push(pos);
    }

    let total_distance = path.len();

    assert_eq!(
        total_distance,
        input.chars().filter(|&c| c == '.').count() + 2
    );

    let mut cheats = FxHashMap::default();
    for p in path {
        // Only makes sense to make moves in the same direction
        for (dx, dy) in [(0, 2), (0, -2), (2, 0), (-2, 0)] {
            let cheat_pos = (p.0 + dx, p.1 + dy);

            if let Some(v) = path_nodes.get(&cheat_pos) {
                let saving = v - path_nodes.get(&p).unwrap() - 2;
                if saving > 0 {
                    cheats
                        .entry(saving)
                        .and_modify(|num| *num += 1)
                        .or_insert(1);
                }
            }
        }
    }

    Some(
        cheats
            .iter()
            .map(|(&k, &v)| if k >= limit { v } else { 0 })
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one_with_limit(input, 100)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_with_limit(&advent_of_code::template::read_file("examples", DAY), 1);
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
