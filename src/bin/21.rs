use std::collections::HashMap;

use fxhash::FxHashMap;
use itertools::Itertools;
use nom::{
    character::complete::{newline, one_of},
    combinator::opt,
    multi::many1,
    sequence::terminated,
    IResult,
};

advent_of_code::solution!(21);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn get_relative_pos(&self, x: usize, y: usize, w: usize, h: usize) -> Option<(usize, usize)> {
        match *self {
            Self::Up => {
                if y > 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            }
            Self::Down => {
                if y < h - 1 {
                    Some((x, y + 1))
                } else {
                    None
                }
            }
            Self::Left => {
                if x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            }
            Self::Right => {
                if x < w - 1 {
                    Some((x + 1, y))
                } else {
                    None
                }
            }
        }
    }

    fn as_key(&self) -> String {
        match *self {
            Direction::Up => "^".to_string(),
            Direction::Down => "v".to_string(),
            Direction::Left => "<".to_string(),
            Direction::Right => ">".to_string(),
        }
    }
}

const NUMERIC_LAYOUT: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['.', '0', 'A'],
];

const DIRECTIONAL_LAYOUT: [[char; 3]; 2] = [['.', '^', 'A'], ['<', 'v', '>']];

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    many1(terminated(many1(one_of("0123456789A")), opt(newline)))(input)
}

fn process_robots(input: &str, num_robots: usize) -> Option<u64> {
    let (_, codes) = parse_input(input).unwrap();

    let (numeric_map, _) = map_key_moves(&NUMERIC_LAYOUT[..]);
    let (direction_map, direction_cost) = map_key_moves(&DIRECTIONAL_LAYOUT[..]);

    let numeric_map = get_optimised_map(numeric_map, &direction_cost);
    let direction_map = get_optimised_map(direction_map, &direction_cost);

    let mut cache = FxHashMap::default();

    let mut total = 0;
    // We need to go depth first, we can more effectively cache results when there is a large number of elements to iterate
    for code in codes {
        let best_seq_length = calculate_score(
            &code.iter().collect(),
            num_robots + 1,
            &numeric_map,
            &direction_map,
            &mut cache,
        );

        let code_number = code[..code.len() - 1]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        total += best_seq_length * code_number;
    }

    Some(total as u64)
}

fn calculate_score(
    code: &String,
    depth: usize,
    layout: &HashMap<(char, char), Vec<String>>,
    directional_map: &HashMap<(char, char), Vec<String>>,
    cache: &mut FxHashMap<(String, usize), usize>,
) -> usize {
    if depth == 0 {
        return code.len();
    }

    let cache_key = (code.clone(), depth);
    if let Some(&score) = cache.get(&cache_key) {
        return score;
    }

    let mut prev_key = 'A';
    let mut best_seq_length = 0;
    for key in code.chars() {
        let options = &layout[&(prev_key, key)];

        best_seq_length += options
            .iter()
            .map(|option| {
                calculate_score(option, depth - 1, directional_map, directional_map, cache)
            })
            .min()
            .unwrap();

        prev_key = key;
    }
    cache.insert(cache_key, best_seq_length);
    best_seq_length
}

fn get_cost(option: &String, direction_cost: &HashMap<(char, char), usize>) -> usize {
    let mut cost = 0;
    for i in 1..option.len() {
        let v = option[i - 1..=i].as_bytes();
        let v1 = v[0] as char;
        let v2 = v[1] as char;
        if v1 != v2 {
            cost += direction_cost[&(v1, v2)];
        }
    }
    cost
}

fn map_key_moves(
    layout: &[[char; 3]],
) -> (
    HashMap<(char, char), Vec<String>>,
    HashMap<(char, char), usize>,
) {
    let mut map: HashMap<char, HashMap<char, Vec<String>>> = HashMap::default();

    let w = layout[0].len();
    let h = layout.len();

    for row in 0..h {
        let r = &layout[row];
        assert_eq!(r.len(), w);

        for col in 0..w {
            let from_key = layout[row][col];
            if from_key == '.' {
                continue;
            }

            map.insert(from_key, HashMap::default());

            for dir in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                if let Some((x, y)) = dir.get_relative_pos(col, row, w, h) {
                    let to_key = layout[y][x];
                    if to_key != '.' {
                        map.get_mut(&from_key)
                            .unwrap()
                            .insert(to_key, vec![dir.as_key()]);
                    }
                }
            }
        }
    }

    // Try all possible distances of movements
    for _ in 2..=(w + h - 2) {
        let orig_map = map.clone();
        for (&from, m) in &orig_map {
            for (to, v) in m {
                if let Some(paths) = orig_map.get(&to) {
                    for (&new_to, path) in paths {
                        let from_map = map.get_mut(&from).unwrap();
                        if new_to != from && !m.contains_key(&new_to) {
                            for s in v {
                                for p in path {
                                    from_map
                                        .entry(new_to)
                                        .and_modify(|v| {
                                            let result = s.to_owned() + p;
                                            if !v.contains(&result) {
                                                v.push(result);
                                            }
                                        })
                                        .or_insert(vec![s.to_owned() + p]);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut result_map = HashMap::default();
    let mut cost_map = HashMap::default();
    for (&from, to_map) in &map {
        for (&to, v) in to_map {
            let cost = v[0].len();
            assert!(v.iter().all(|i| i.len() == cost));
            cost_map.insert((from, to), cost);
            result_map.insert((from, to), v.iter().map(|i| i.to_owned() + "A").collect());
        }
        result_map.insert((from, from), vec!["A".to_string()]);
    }
    (result_map, cost_map)
}

fn get_optimised_map(
    map: HashMap<(char, char), Vec<String>>,
    cost_map: &HashMap<(char, char), usize>,
) -> HashMap<(char, char), Vec<String>> {
    let mut optimised_map = HashMap::default();
    for (k, v) in map {
        let s = v.into_iter().min_set_by_key(|i| get_cost(i, &cost_map));
        optimised_map.insert(k, s);
    }

    optimised_map
}

pub fn part_one(input: &str) -> Option<u64> {
    process_robots(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    process_robots(input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_key_moves() {
        let keys = [['1', '2', '.'], ['.', 'A', '.']];

        let expected_map = HashMap::from_iter([
            (('1', '1'), vec!["A".to_string()]),
            (('1', '2'), vec![">A".to_string()]),
            (('1', 'A'), vec![">vA".to_string()]),
            (('2', '1'), vec!["<A".to_string()]),
            (('2', 'A'), vec!["vA".to_string()]),
            (('2', '2'), vec!["A".to_string()]),
            (('A', '1'), vec!["^<A".to_string()]),
            (('A', '2'), vec!["^A".to_string()]),
            (('A', 'A'), vec!["A".to_string()]),
        ]);
        let expected_cost = HashMap::from_iter([
            (('1', '2'), 1),
            (('1', 'A'), 2),
            (('2', '1'), 1),
            (('2', 'A'), 1),
            (('A', '1'), 2),
            (('A', '2'), 1),
        ]);

        let (map, cost) = map_key_moves(&keys[..]);
        assert_eq!(expected_map, map);
        assert_eq!(expected_cost, cost);

        let (result, cost) = map_key_moves(&DIRECTIONAL_LAYOUT[..]);
        let result = get_optimised_map(result, &cost);
        assert_eq!(result[&('<', '^')], vec![">^A"]);
        assert_eq!(result[&('>', '^')], vec!["<^A"]);

        let (result, _) = map_key_moves(&NUMERIC_LAYOUT[..]);
        let result = get_optimised_map(result, &cost);
        assert_eq!(
            result[&('2', '9')].iter().sorted().collect_vec(),
            vec![">^^A", "^^>A"]
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
