use nom::{
    character::complete::{newline, u64},
    combinator::opt,
    multi::many1,
    sequence::terminated,
    IResult,
};

advent_of_code::solution!(22);

fn parse_input(input: &str) -> IResult<&str, Vec<u64>> {
    many1(terminated(u64, opt(newline)))(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, buyers) = parse_input(input).unwrap();

    Some(buyers.iter().map(|&seed| nth_secret(seed, 2000)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn mix_secret(secret: u64, mix: u64) -> u64 {
    secret ^ mix
}

fn prune_secret(secret: u64) -> u64 {
    secret % 16777216
}

fn next_secret(secret: u64) -> u64 {
    let n = secret * 64;
    let secret = prune_secret(mix_secret(secret, n));

    let n = secret / 32;
    let secret = prune_secret(mix_secret(secret, n));

    let n = secret * 2048;
    prune_secret(mix_secret(secret, n))
}

fn nth_secret(secret: u64, n: usize) -> u64 {
    let mut next = secret;
    for _ in 0..n {
        next = next_secret(next);
    }
    next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_number() {
        assert_eq!(mix_secret(42, 15), 37);
        assert_eq!(prune_secret(100000000), 16113920);
        assert_eq!(next_secret(123), 15887950);

        for (n, s) in [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ]
        .into_iter()
        .enumerate()
        {
            assert_eq!(nth_secret(123, n + 1), s);
        }

        assert_eq!(nth_secret(1, 2000), 8685429);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
