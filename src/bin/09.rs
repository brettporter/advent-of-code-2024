use itertools::Itertools;
use nom::{character::complete::digit1, IResult, InputIter};

advent_of_code::solution!(9);

fn parse_input(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, disk_map) = parse_input(input).unwrap();

    let disk_map = disk_map
        .iter_elements()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect_vec();

    let blocks = disk_map
        .iter()
        .enumerate()
        .filter_map(|(i, &count)| if i % 2 == 0 { Some(count) } else { None })
        .collect_vec();

    let mut pull_from = blocks.len() - 1;
    let mut pull_remaining = blocks[pull_from];

    let mut checksum = 0u64;
    let mut pos = 0u64;

    for (idx, &count) in blocks.iter().enumerate() {
        println!("Allocating: {idx}, pulling from {pull_from}");
        if idx > pull_from {
            break;
        }
        if idx == pull_from {
            for i in 0..pull_remaining {
                checksum += (pos + i) * idx as u64;
            }
            break;
        }

        for i in 0..count {
            checksum += (pos + i) * idx as u64;
        }
        pos += count;

        // fill space
        let mut space = disk_map[idx * 2 + 1];
        println!("Space to fill {space} with pull_remaining {pull_remaining}");
        while space > 0 {
            let pull = space.min(pull_remaining);
            for i in 0..pull {
                checksum += (pos + i) * pull_from as u64;
            }
            pos += pull;
            pull_remaining -= pull;
            space -= pull;

            if space > 0 {
                pull_from -= 1;
                pull_remaining = blocks[pull_from];
                if idx == pull_from {
                    break;
                }
            }
        }
    }

    Some(checksum)
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
