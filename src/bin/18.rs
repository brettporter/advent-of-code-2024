use fxhash::FxHashMap;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::opt,
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};
use priority_queue::DoublePriorityQueue;

advent_of_code::solution!(18);

fn parse_input(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    many1(terminated(separated_pair(i32, tag(","), i32), opt(newline)))(input)
}

fn simulate_corruption(input: &str, num_entries: usize, size: usize) -> Vec<Vec<bool>> {
    let (_, coordinates) = parse_input(input).unwrap();

    let coordinates = &coordinates[0..num_entries];

    let mut row = Vec::with_capacity(size);
    row.resize(size, false);

    let mut grid = Vec::with_capacity(size);
    for _ in 0..size {
        grid.push(row.clone());
    }

    for &(x, y) in coordinates {
        grid[y as usize][x as usize] = true
    }

    grid
}

fn dijkstra(grid: &Vec<Vec<bool>>, start: (usize, usize), end: (usize, usize)) -> u32 {
    let size = grid.len();

    let mut queue = DoublePriorityQueue::new();
    queue.push(start, 0);

    let mut cost_tally = FxHashMap::default();
    cost_tally.insert(start, 0);

    while let Some((cur_pos, cost)) = queue.pop_min() {
        if cur_pos == end {
            return cost;
        }

        let options = get_options(cur_pos, size);
        let valid_options = options
            .iter()
            .filter(|pos| grid[pos.1][pos.0] != true)
            .collect_vec();

        for &n in valid_options {
            let score = cost + 1;
            if score <= *cost_tally.get(&n).unwrap_or(&u32::MAX) {
                cost_tally.insert(n, score);
                queue.push(n, score);
            }
        }
    }

    unreachable!("Did not find a path to end");
}

fn get_options(pos: (usize, usize), size: usize) -> Vec<(usize, usize)> {
    let mut options = vec![];
    if pos.1 > 0 {
        options.push((pos.0, pos.1 - 1));
    }
    if pos.1 < size - 1 {
        options.push((pos.0, pos.1 + 1));
    }
    if pos.0 > 0 {
        options.push((pos.0 - 1, pos.1));
    }
    if pos.0 < size - 1 {
        options.push((pos.0 + 1, pos.1));
    }
    options
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one_with_size(input, 1024, 71)
}

fn part_one_with_size(input: &str, num_entries: usize, size: usize) -> Option<u32> {
    let grid = simulate_corruption(input, num_entries, size);

    let start = (0, 0);
    let end = (size - 1, size - 1);

    let cost = dijkstra(&grid, start, end);

    Some(cost)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one_with_size(&advent_of_code::template::read_file("examples", DAY), 12, 7);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
