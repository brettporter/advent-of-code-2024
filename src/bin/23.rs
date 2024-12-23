use fxhash::{FxHashMap, FxHashSet};
use nom::{
    character::complete::{alpha1, char, newline},
    combinator::opt,
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

advent_of_code::solution!(23);

fn parse_input(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    many1(terminated(
        separated_pair(alpha1, char('-'), alpha1),
        opt(newline),
    ))(input)
}

fn map_network(connections: &[(&str, &str)]) -> FxHashMap<String, Vec<String>> {
    let mut network = FxHashMap::default();
    for &(c1, c2) in connections {
        network
            .entry(c1.to_string())
            .and_modify(|v: &mut Vec<String>| v.push(c2.to_string()))
            .or_insert(vec![c2.to_string()]);
        network
            .entry(c2.to_string())
            .and_modify(|v: &mut Vec<String>| v.push(c1.to_string()))
            .or_insert(vec![c1.to_string()]);
    }
    network
}

fn find_interconnected(network: &FxHashMap<String, Vec<String>>) -> FxHashSet<Vec<String>> {
    let mut result = FxHashSet::default();
    for (c1, v) in network {
        for c2 in v {
            for c3 in &network[c2] {
                if network[c3].contains(c1) {
                    let mut interconnected = vec![c1.clone(), c2.clone(), c3.clone()];
                    interconnected.sort();
                    result.insert(interconnected);
                }
            }
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, connections) = parse_input(input).unwrap();

    let network = map_network(&connections);

    let interconnected = find_interconnected(&network);

    Some(
        interconnected
            .iter()
            .filter(|v| v.iter().any(|s| s.starts_with('t')))
            .count() as u32,
    )
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
