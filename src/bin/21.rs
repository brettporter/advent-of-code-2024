use std::{collections::HashMap, option};

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

pub fn part_one(input: &str) -> Option<u32> {
    let (_, codes) = parse_input(input).unwrap();

    let (numeric_map, _) = map_key_moves(&NUMERIC_LAYOUT[..]);
    let (direction_map, direction_cost) = map_key_moves(&DIRECTIONAL_LAYOUT[..]);

    let mut total = 0;
    // TODO: don't need all of the options
    for code in codes {
        let options = get_options(&code, &numeric_map);

        let new_options = get_cheapest_option(&options, &direction_cost);

        let options = new_options
            .iter()
            .flat_map(|option| get_options(&option.chars().collect_vec(), &direction_map))
            .collect_vec();
        let new_options = get_cheapest_option(&options, &direction_cost);

        let options = new_options
            .iter()
            .flat_map(|option| get_options(&option.chars().collect_vec(), &direction_map))
            .collect_vec();
        // TODO: doesn't have to be lowest next cost at this stage
        // let new_options = get_cheapest_option(&options, &direction_cost);
        let option = options.first().unwrap();

        let code_number = code[..code.len() - 1]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        total += option.len() * code_number;
    }

    Some(total as u32)
}

fn get_cheapest_option(
    options: &[String],
    direction_cost: &HashMap<(char, char), usize>,
) -> Vec<String> {
    let mut new_options = vec![];
    let mut best_cost = usize::MAX;
    for option in options {
        let mut cost = 0;
        for i in 1..option.len() {
            let mut c = option[i - 1..=i].chars();
            let v1 = c.next().unwrap();
            let v2 = c.next().unwrap();
            if v1 != v2 {
                cost += direction_cost[&(v1, v2)];
            }
        }

        if cost == best_cost {
            new_options.push(option.to_owned());
        } else if cost < best_cost {
            best_cost = cost;
            new_options = vec![option.to_owned()];
        }
    }
    new_options
}

fn get_options(code: &[char], map: &HashMap<char, HashMap<char, Vec<String>>>) -> Vec<String> {
    let mut cur_options: Vec<String> = vec!["".to_string()];
    let mut cur_key = 'A';
    for &key in code {
        let mut options = vec![];
        for o in cur_options {
            if cur_key == key {
                options.push(o + "A");
            } else {
                for v in &map[&cur_key][&key] {
                    options.push(o.clone() + v + "A");
                }
            }
        }
        cur_key = key;
        cur_options = options;
    }
    cur_options
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn map_key_moves(
    layout: &[[char; 3]],
) -> (
    HashMap<char, HashMap<char, Vec<String>>>,
    HashMap<(char, char), usize>,
) {
    let mut map = HashMap::default();

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

    let mut cost_map = HashMap::default();
    for (&from, to_map) in &map {
        for (&to, v) in to_map {
            let cost = v[0].len();
            assert!(v.iter().all(|i| i.len() == cost));
            cost_map.insert((from, to), cost);
        }
    }

    (map, cost_map)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_map_key_moves() {
        let keys = [['1', '2', '.'], ['.', 'A', '.']];
        let expected_map = HashMap::from_iter([
            (
                '1',
                HashMap::from_iter([('2', vec![">".to_string()]), ('A', vec![">v".to_string()])]),
            ),
            (
                '2',
                HashMap::from_iter([('A', vec!["v".to_string()]), ('1', vec!["<".to_string()])]),
            ),
            (
                'A',
                HashMap::from_iter([('1', vec!["^<".to_string()]), ('2', vec!["^".to_string()])]),
            ),
        ]);
        let expected_cost = HashMap::from_iter([
            (('1', '2'), 1),
            (('1', 'A'), 2),
            (('2', '1'), 1),
            (('2', 'A'), 1),
            (('A', '1'), 2),
            (('A', '2'), 1),
        ]);

        assert_eq!((expected_map, expected_cost), map_key_moves(&keys[..]));

        let (result, _) = map_key_moves(&DIRECTIONAL_LAYOUT[..]);
        assert_eq!(result[&'<'][&'^'], vec![">^"]);
        assert_eq!(
            result[&'>'][&'^'].iter().sorted().collect_vec(),
            vec!["<^", "^<"]
        );

        let (result, _) = map_key_moves(&NUMERIC_LAYOUT[..]);
        assert_eq!(
            result[&'2'][&'9'].iter().sorted().collect_vec(),
            vec![">^^", "^>^", "^^>"]
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
        assert_eq!(result, None);
    }
}
