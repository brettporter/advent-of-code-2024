use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

advent_of_code::solution!(2);

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    many1(terminated(separated_list1(tag(" "), i32), opt(newline)))(input)
}

fn is_safe(report: &[i32]) -> bool {
    // Check the sign of the first two numbers, to ensure it is consistent throughout
    let sign = (report[1] - report[0]).signum();

    // For each number, check the sign is the same (consistently increasing or decreasing)
    // and check the difference is between 1 and 3 inclusive (regardless of sign)
    for i in 1..report.len() {
        if (report[i] - report[i - 1]).signum() != sign {
            return false;
        } else if !(1..=3).contains(&report[i].abs_diff(report[i - 1])) {
            return false;
        }
    }
    true
}

fn is_safe_with_dampener(report: &Vec<i32>) -> bool {
    // Check if either the original array is safe, or if removing a single element makes it safe
    // by iterating through all index numbers and testing for a slice that removes that index
    // returning the first time it encounters a safe report
    is_safe(report)
        || (0..report.len()).any(|i| is_safe(&[&report[0..i], &report[i + 1..]].concat()))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, reports) = parse_input(input).unwrap();

    // Count the number of safe reports in the input
    Some(reports.iter().filter(|v| is_safe(v)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, reports) = parse_input(input).unwrap();

    // Count the number of safe reports in the input, when calculated using a dampener for excluding any one element
    Some(reports.iter().filter(|v| is_safe_with_dampener(v)).count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe() {
        assert_eq!(true, is_safe(&vec![7, 6, 4, 2, 1]));
        assert_eq!(false, is_safe(&vec![1, 2, 7, 8, 9]));
        assert_eq!(false, is_safe(&vec![9, 7, 6, 2, 1]));
        assert_eq!(false, is_safe(&vec![1, 3, 2, 4, 5]));
        assert_eq!(false, is_safe(&vec![8, 6, 4, 4, 1]));
        assert_eq!(true, is_safe(&vec![1, 3, 6, 7, 9]));
    }

    #[test]
    fn test_safe_with_dampener() {
        assert_eq!(true, is_safe_with_dampener(&vec![7, 6, 4, 2, 1]));
        assert_eq!(false, is_safe_with_dampener(&vec![1, 2, 7, 8, 9]));
        assert_eq!(false, is_safe_with_dampener(&vec![9, 7, 6, 2, 1]));
        assert_eq!(true, is_safe_with_dampener(&vec![1, 3, 2, 4, 5]));
        assert_eq!(true, is_safe_with_dampener(&vec![8, 6, 4, 4, 1]));
        assert_eq!(true, is_safe_with_dampener(&vec![1, 3, 6, 7, 9]));

        // Input data that failed
        assert_eq!(true, is_safe_with_dampener(&vec![66, 67, 68, 71, 75]));
        assert_eq!(
            false,
            is_safe_with_dampener(&vec![39, 41, 41, 42, 44, 46, 49, 46])
        );
        assert_eq!(
            true,
            is_safe_with_dampener(&vec![84, 82, 83, 84, 85, 88, 90])
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
