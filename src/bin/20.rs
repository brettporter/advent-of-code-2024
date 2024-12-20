use fxhash::FxHashMap;
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

fn find_cheats_local_strategy(input: &str, limit: usize) -> Option<u32> {
    let path = build_path(input);

    let mut count = 0;
    let path_positions = FxHashMap::from_iter(path.iter().enumerate().map(|(i, p)| (p, i)));

    for (i, p) in path.iter().enumerate() {
        // Only makes sense to make moves in the same direction
        for (dx, dy) in [(0, 2), (0, -2), (2, 0), (-2, 0)] {
            let cheat_pos = (p.0 + dx, p.1 + dy);

            if let Some(&v) = path_positions.get(&cheat_pos) {
                if v >= i + limit + 2 {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

fn build_path(input: &str) -> Vec<(i32, i32)> {
    let (_, grid) = parse_input(input).unwrap();

    let start = find_item(&grid, 'S');

    // Navigate S -> E and save the distance from start, will use this repeat with cheats

    let mut pos = start;
    let mut last = start;
    let mut path = vec![start];

    // Traverse the main path, which we know there is only a single end to end solution
    // Collect into a path to walk, and map each location to its distance
    while grid[pos.1 as usize][pos.0 as usize] != 'E' {
        let (dx, dy) = [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .iter()
            .find(|&&p| {
                let next_pos = (pos.0 + p.0, pos.1 + p.1);
                next_pos != last && grid[next_pos.1 as usize][next_pos.0 as usize] != '#'
            })
            .unwrap();

        last = pos;
        pos = (pos.0 + dx, pos.1 + dy);
        path.push(pos);
    }
    path
}

fn find_cheats_path_strategy(input: &str, max_length: u32, limit: usize) -> Option<u32> {
    let path = build_path(input);

    // Assemble all valid cheats
    // Re-walk the path, and at each location, find all the destination locations that can be reached
    // in the given number of picoseconds by trying each subsequent destination on the path
    // that is reachable within cheat length
    let mut count = 0;
    for (i, p1) in path.iter().enumerate() {
        if i + limit < path.len() {
            for (j, p2) in path[i + limit..].iter().enumerate() {
                let cheat_length = p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1);
                if cheat_length <= max_length && j >= cheat_length as usize {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

fn part_one_with_limit(input: &str, limit: usize) -> Option<u32> {
    find_cheats_local_strategy(input, limit)
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one_with_limit(input, 100)
}

pub fn part_two_with_limit(input: &str, limit: usize) -> Option<u32> {
    find_cheats_path_strategy(input, 20, limit)
}

pub fn part_two(input: &str) -> Option<u32> {
    part_two_with_limit(input, 100)
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
        let result = part_two_with_limit(&advent_of_code::template::read_file("examples", DAY), 50);
        assert_eq!(result, Some(285));
    }
}
