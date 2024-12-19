use fxhash::FxHashMap;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

advent_of_code::solution!(19);

fn parse_input(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (input, patterns) = terminated(separated_list1(tag(", "), alpha1), newline)(input)?;
    let (input, _) = newline(input)?;
    let (input, designs) = many1(terminated(alpha1, opt(newline)))(input)?;
    Ok((input, (patterns, designs)))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (patterns, designs)) = parse_input(input).unwrap();

    let mut cache = FxHashMap::default();

    Some(
        designs
            .iter()
            .filter(|d| get_possible_designs(d, &patterns, false, &mut cache) > 0)
            .count() as u32,
    )
}

fn get_possible_designs(
    design: &str,
    patterns: &Vec<&str>,
    all: bool,
    cache: &mut FxHashMap<String, u64>,
) -> u64 {
    if design.is_empty() {
        return 1;
    }
    if let Some(&v) = cache.get(&design.to_string()) {
        return v;
    }

    let mut total = 0;
    for p in patterns {
        if design.starts_with(p) {
            total += get_possible_designs(&design[p.len()..], patterns, all, cache);
            if !all && total > 0 {
                cache.insert(design.to_string(), 1);
                return 1;
            }
        }
    }

    cache.insert(design.to_string(), total);
    total
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (patterns, designs)) = parse_input(input).unwrap();

    let mut cache = FxHashMap::default();

    Some(
        designs
            .iter()
            .map(|d| get_possible_designs(d, &patterns, true, &mut cache))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
