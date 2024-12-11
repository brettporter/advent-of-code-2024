use fxhash::FxHashMap;
use nom::{bytes::complete::tag, character::complete::u64, multi::separated_list1, IResult};

advent_of_code::solution!(11);

fn parse_input(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(" "), u64)(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    // Naïve approach!
    let (_, numbers) = parse_input(input).unwrap();

    Some(count_all_stones(&numbers, 25))
}

fn count_all_stones(numbers: &[u64], it: i32) -> u64 {
    let mut stones = FxHashMap::default();
    for &n in numbers {
        stones.insert(n, 1);
    }

    for _ in 0..it {
        let mut new_stones = FxHashMap::default();
        for (n, count) in stones {
            if n == 0 {
                new_stones
                    .entry(1)
                    .and_modify(|n| *n += count)
                    .or_insert(count);
            } else {
                let digits = n.ilog10() + 1;
                if digits % 2 == 0 {
                    let p = 10u64.pow(digits / 2);
                    new_stones
                        .entry(n / p)
                        .and_modify(|n| *n += count)
                        .or_insert(count);
                    new_stones
                        .entry(n % p)
                        .and_modify(|n| *n += count)
                        .or_insert(count);
                } else {
                    new_stones
                        .entry(n * 2024)
                        .and_modify(|n| *n += count)
                        .or_insert(count);
                }
            }
        }
        stones = new_stones;
    }

    stones.iter().map(|(_, v)| v).sum()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, numbers) = parse_input(input).unwrap();

    Some(count_all_stones(&numbers, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_stones() {
        assert_eq!(7, count_all_stones(&vec![0, 1, 10, 99, 999], 1));
        assert_eq!(3, count_all_stones(&vec![125, 17], 1));
        assert_eq!(4, count_all_stones(&vec![125, 17], 2));
        assert_eq!(5, count_all_stones(&vec![125, 17], 3));
        assert_eq!(9, count_all_stones(&vec![125, 17], 4));
        assert_eq!(13, count_all_stones(&vec![125, 17], 5));
        assert_eq!(22, count_all_stones(&vec![125, 17], 6));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
