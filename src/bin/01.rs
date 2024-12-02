use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32},
    combinator::opt,
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (_, values) = parse_input(input).unwrap();

    // Transpose rows into columns, and sort these lists independently of each other to pair up the smallest each time
    let (mut list1, mut list2): (Vec<u32>, Vec<u32>) = values.into_iter().unzip();

    list1.sort();
    list2.sort();

    // Add the difference of each pair in the lists
    Some(
        list1
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, &e)| acc + e.abs_diff(list2[idx])),
    )
}

fn parse_input(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    many1(terminated(
        separated_pair(u32, tag("   "), u32),
        opt(newline),
    ))(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, values) = parse_input(input).unwrap();

    // Transpose rows into columns
    let (list1, list2): (Vec<u32>, Vec<u32>) = values.into_iter().unzip();

    // For each number in the first list, count the number in the second list then multiple by the original value
    // Add these together to calculate the total
    Some(list1.iter().fold(0, |acc, &e| {
        acc + (list2.iter().filter(|&&x| x == e).count() as u32) * e
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
