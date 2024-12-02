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

fn is_safe(report: &Vec<i32>) -> bool {
    let sign = (report[1] - report[0]).signum();

    for i in 1..report.len() {
        if check_items(report[i], report[i - 1], sign) == false {
            return false;
        }
    }
    true
}

fn check_items(cur: i32, prev: i32, sign: i32) -> bool {
    if (cur - prev).signum() != sign {
        return false;
    } else if !(1..=3).contains(&cur.abs_diff(prev)) {
        return false;
    }
    true
}

fn is_safe_with_dampener(report: &Vec<i32>) -> bool {
    /*
    let sign = (report[1] - report[0]).signum();
    for i in 1..report.len() {
        if check_items(report[i], report[i - 1], sign) == false {
            if i > 1 && check_items(report[i], report[i - 2], sign) == false {
                return false;
            }
        }
    }
    true
            */

    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut v = report.clone();
        v.remove(i);
        if is_safe(&v) {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, reports) = parse_input(input).unwrap();

    Some(reports.into_iter().filter(|v| is_safe(v)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, reports) = parse_input(input).unwrap();

    Some(
        reports
            .into_iter()
            .filter(|v| is_safe_with_dampener(v))
            .count() as u32,
    )
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
