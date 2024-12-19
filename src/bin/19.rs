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

#[derive(PartialEq)]
enum Mode {
    AllArrangements,
    AnyPossible,
}

fn parse_input(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (input, patterns) = terminated(separated_list1(tag(", "), alpha1), newline)(input)?;
    let (input, _) = newline(input)?;
    let (input, designs) = many1(terminated(alpha1, opt(newline)))(input)?;
    Ok((input, (patterns, designs)))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (patterns, designs)) = parse_input(input).unwrap();

    let mut cache = FxHashMap::default();

    // Count all the designs that are possible
    // Short-circuit when a possible design is found by setting all to false
    Some(
        designs
            .iter()
            .filter(|d| get_possible_designs(d, &patterns, &Mode::AnyPossible, &mut cache) > 0)
            .count() as u32,
    )
}

fn get_possible_designs(
    design: &str,
    patterns: &Vec<&str>,
    mode: &Mode,
    cache: &mut FxHashMap<String, u64>,
) -> u64 {
    // If we have gotten to the end of a design string, this is a valid arrangement - return 1 to add to the count
    if design.is_empty() {
        return 1;
    }
    // If we have already found the count for this design, return from cache
    if let Some(&v) = cache.get(&design.to_string()) {
        return v;
    }

    // For each pattern that applies to the start of the current portion of the design,
    // recursively find all the arrangements for the remainder of the design after that pattern
    // and add them to the total
    let mut total = 0;
    for p in patterns {
        if design.starts_with(p) {
            total += get_possible_designs(&design[p.len()..], patterns, mode, cache);
            // if short circuiting before getting all, just return 1 to show it is possible
            if *mode == Mode::AnyPossible && total > 0 {
                cache.insert(design.to_string(), 1);
                return 1;
            }
        }
    }

    // Cache the number of arrangements found
    cache.insert(design.to_string(), total);
    total
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (patterns, designs)) = parse_input(input).unwrap();

    let mut cache = FxHashMap::default();

    // Count the number of valid arrangements for each design
    Some(
        designs
            .iter()
            .map(|d| get_possible_designs(d, &patterns, &Mode::AllArrangements, &mut cache))
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
