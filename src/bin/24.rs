use std::{collections::VecDeque, str::FromStr};

use fxhash::FxHashMap;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline, u8},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

advent_of_code::solution!(24);

#[derive(Debug)]
enum Operation {
    AND,
    OR,
    XOR,
}
impl Operation {
    fn perform(&self, in_lhs_gate: u8, in_rhs_gate: u8) -> u8 {
        match *self {
            Operation::AND => in_lhs_gate & in_rhs_gate,
            Operation::OR => in_lhs_gate | in_rhs_gate,
            Operation::XOR => in_lhs_gate ^ in_rhs_gate,
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Operation, Self::Err> {
        match s {
            "AND" => Ok(Operation::AND),
            "OR" => Ok(Operation::OR),
            "XOR" => Ok(Operation::XOR),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Circuit {
    in_lhs_gate: String,
    in_rhs_gate: String,
    op: Operation,
    out_gate: String,
}

fn parse_input(input: &str) -> IResult<&str, (FxHashMap<String, u8>, Vec<Circuit>)> {
    let (input, state) = many1(terminated(
        separated_pair(alphanumeric1, tag(": "), u8),
        opt(newline),
    ))(input)?;
    let (input, _) = newline(input)?;
    let (input, circuits) = many1(terminated(
        separated_pair(
            separated_list1(tag(" "), alphanumeric1),
            tag(" -> "),
            alphanumeric1,
        ),
        opt(newline),
    ))(input)?;

    let initial_state = FxHashMap::from_iter(state.into_iter().map(|(k, v)| (k.to_string(), v)));

    let circuits = circuits
        .iter()
        .map(|(iv, o)| Circuit {
            in_lhs_gate: iv[0].to_string(),
            in_rhs_gate: iv[2].to_string(),
            op: Operation::from_str(iv[1]).unwrap(),
            out_gate: o.to_string(),
        })
        .collect();

    Ok((input, (initial_state, circuits)))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, (mut state, circuits)) = parse_input(input).unwrap();

    let mut waiting_circuits = VecDeque::from_iter(circuits);

    while let Some(c) = waiting_circuits.pop_front() {
        let v1 = state.get(&c.in_lhs_gate);
        let v2 = state.get(&c.in_rhs_gate);
        if v1.is_some() && v2.is_some() {
            state.insert(c.out_gate, c.op.perform(*v1.unwrap(), *v2.unwrap()));
        } else {
            waiting_circuits.push_back(c);
        }
    }

    Some(
        state
            .into_iter()
            .filter(|(k, _)| k.starts_with('z'))
            .fold(0, |acc, (k, v)| {
                let idx: u8 = k[1..].parse().unwrap();
                acc | ((v as u64) << idx)
            }),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 11,
        ));
        assert_eq!(result, Some(4));

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
