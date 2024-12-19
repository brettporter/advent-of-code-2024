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
            .filter(|d| is_possible_design(d, &patterns, &mut cache))
            .count() as u32,
    )
}

fn is_possible_design(
    design: &str,
    patterns: &Vec<&str>,
    cache: &mut FxHashMap<String, bool>,
) -> bool {
    if design.is_empty() {
        return true;
    }
    if let Some(&v) = cache.get(&design.to_string()) {
        return v;
    }

    for p in patterns {
        if design.starts_with(p) && is_possible_design(&design[p.len()..], patterns, cache) {
            cache.insert(design.to_string(), true);
            return true;
        }
    }

    cache.insert(design.to_string(), false);
    false
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
