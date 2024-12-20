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

fn find_cheats(input: &str, max_length: i32, limit: i32) -> Option<u32> {
    let (_, grid) = parse_input(input).unwrap();

    let size = grid.len() as i32;
    assert_eq!(grid[0].len() as i32, size);

    let start = find_item(&grid, 'S');

    // Navigate S -> E and save the distance from start, will use this repeat with cheats

    let mut pos = start;
    let mut last = start;
    let mut path_nodes = FxHashMap::from_iter([(start, 0)]);
    let mut path = vec![start];

    // Traverse the main path, which we know there is only a single end to end solution
    // Collect into a path to walk, and map each location to its distance
    while grid[pos.1 as usize][pos.0 as usize] != 'E' {
        let (dx, dy) = [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .iter()
            .find(|&&p| {
                let next_pos = (pos.0 + p.0, pos.1 + p.1);
                next_pos != last
                    && next_pos.0 >= 0
                    && next_pos.0 < size
                    && next_pos.1 >= 0
                    && next_pos.1 < size
                    && grid[next_pos.1 as usize][next_pos.0 as usize] != '#'
            })
            .unwrap();

        last = pos;
        pos = (pos.0 + dx, pos.1 + dy);
        path_nodes.insert(pos, path_nodes.len() as i32);
        path.push(pos);
    }

    // Assemble all valid cheats
    // Re-walk the path, and at each location, find all the destination locations that can be reached
    // in the given number of picoseconds that is not a wall by trying each possible combination of
    // x & y that add to 20 in each quadrant surrounding this point
    // If that location is on the path, save that cheat along with its time saving, which is the distance
    // on the path between them, less the time spent executing the cheat
    let mut cheats = FxHashMap::default();
    for p in path {
        for dy in 0..=max_length {
            for dx in 0..=(max_length - dy) {
                for quadrant in [(dx, dy), (dx, -dy), (-dx, dy), (-dx, -dy)] {
                    let cheat_pos = (p.0 + quadrant.0, p.1 + quadrant.1);
                    let cheat_length = dx + dy;
                    assert!(cheat_length <= max_length);
                    if cheat_length > 0 {
                        if let Some(v) = path_nodes.get(&cheat_pos) {
                            let saving = v - path_nodes.get(&p).unwrap() - cheat_length;
                            if saving > 0 {
                                cheats.insert((p, cheat_pos), saving);
                            }
                        }
                    }
                }
            }
        }
    }

    // Return the number of cheats that save the specified amount of time
    Some(cheats.values().filter(|&&v| v >= limit).count() as u32)
}

fn part_one_with_limit(input: &str, limit: i32) -> Option<u32> {
    find_cheats(input, 2, limit)
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one_with_limit(input, 100)
}

pub fn part_two_with_limit(input: &str, limit: i32) -> Option<u32> {
    find_cheats(input, 20, limit)
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
