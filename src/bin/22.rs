use std::collections::HashMap;

use fxhash::FxHashMap;
use nom::{
    character::complete::{i64, newline},
    combinator::opt,
    multi::many1,
    sequence::terminated,
    IResult,
};

advent_of_code::solution!(22);

fn parse_input(input: &str) -> IResult<&str, Vec<i64>> {
    many1(terminated(i64, opt(newline)))(input)
}

pub fn part_one(input: &str) -> Option<i64> {
    let (_, buyers) = parse_input(input).unwrap();

    Some(buyers.iter().map(|&seed| nth_secret(seed, 2000)).sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, buyers) = parse_input(input).unwrap();

    let sequences = generate_buyer_sequences(buyers);

    sequences
        .iter()
        .map(|(_, bananas)| bananas.values().sum())
        .max()
}

fn generate_buyer_sequences(buyers: Vec<i64>) -> FxHashMap<Vec<i64>, HashMap<usize, i64>> {
    let mut sequences = FxHashMap::default();
    for (buyer_idx, &seed) in buyers.iter().enumerate() {
        let mut secret = seed;
        let mut bananas = secret % 10;
        let mut changes = vec![];
        for _ in 0..2000 {
            secret = next_secret(secret);
            let next = secret % 10;
            changes.push(next - bananas);
            bananas = next;

            if changes.len() > 3 {
                let seq = changes[changes.len() - 4..].to_vec();
                sequences
                    .entry(seq)
                    .and_modify(|v: &mut HashMap<usize, i64>| {
                        if !v.contains_key(&buyer_idx) {
                            v.insert(buyer_idx, bananas);
                        }
                    })
                    .or_insert_with(|| HashMap::from_iter([(buyer_idx, bananas)]));
            }
        }
    }
    sequences
}

fn mix_secret(secret: i64, mix: i64) -> i64 {
    secret ^ mix
}

fn prune_secret(secret: i64) -> i64 {
    secret % 16777216
}

fn next_secret(secret: i64) -> i64 {
    let n = secret * 64;
    let secret = prune_secret(mix_secret(secret, n));

    let n = secret / 32;
    let secret = prune_secret(mix_secret(secret, n));

    let n = secret * 2048;
    prune_secret(mix_secret(secret, n))
}

fn nth_secret(secret: i64, n: usize) -> i64 {
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
    fn test_generate_buyer_sequnces() {
        let seq = generate_buyer_sequences(vec![123]);
        assert_eq!(seq[&vec![-1, -1, 0, 2]], HashMap::from([(0, 6)]));

        let seq = generate_buyer_sequences(vec![1, 2, 3, 2024]);
        assert_eq!(
            seq[&vec![-2, 1, -1, 3]],
            HashMap::from([(0, 7), (1, 7), (3, 9)])
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
