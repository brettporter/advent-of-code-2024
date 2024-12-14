use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline},
    combinator::opt,
    multi::many1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

advent_of_code::solution!(13);

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64,
}
impl Coord {
    fn from_tuple(t: (i64, i64)) -> Coord {
        let (x, y) = t;
        Self { x, y }
    }
}

#[derive(Debug)]
struct ClawMachine {
    a_increment: Coord,
    b_increment: Coord,
    prize_location: Coord,
}

fn parse_input(input: &str) -> IResult<&str, Vec<ClawMachine>> {
    many1(terminated(
        tuple((
            terminated(
                preceded(
                    tag("Button A: "),
                    separated_pair(
                        preceded(tag("X+"), i64),
                        tag(", "),
                        preceded(tag("Y+"), i64),
                    ),
                ),
                newline,
            ),
            terminated(
                preceded(
                    tag("Button B: "),
                    separated_pair(
                        preceded(tag("X+"), i64),
                        tag(", "),
                        preceded(tag("Y+"), i64),
                    ),
                ),
                newline,
            ),
            terminated(
                preceded(
                    tag("Prize: "),
                    separated_pair(
                        preceded(tag("X="), i64),
                        tag(", "),
                        preceded(tag("Y="), i64),
                    ),
                ),
                newline,
            ),
        ))
        .map(|(a_increment, b_increment, prize_location)| ClawMachine {
            a_increment: Coord::from_tuple(a_increment),
            b_increment: Coord::from_tuple(b_increment),
            prize_location: Coord::from_tuple(prize_location),
        }),
        opt(newline),
    ))(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, machines) = parse_input(input).unwrap();

    Some(calculate_cost(&machines, Some(100)))
}

fn calculate_cost(machines: &[ClawMachine], limit: Option<i64>) -> u64 {
    let mut total = 0;
    for machine in machines {
        // Given equations:
        //  AX * a + BX * b = PX
        //  AY * a + BY * b = PY
        //
        // Calculate intersection:
        // b = (PY * AX - AY * PX) / (BY * AX - AY * BX)
        // a = (PX - BX * b) / AX
        let b2 = machine.b_increment.y * machine.a_increment.x
            - machine.a_increment.y * machine.b_increment.x;
        let b1 = machine.prize_location.y * machine.a_increment.x
            - machine.a_increment.y * machine.prize_location.x;
        if b1 % b2 == 0 {
            let b = b1 / b2;
            let a1 = machine.prize_location.x - machine.b_increment.x * b;
            if a1 % machine.a_increment.x == 0 {
                let a = a1 / machine.a_increment.x;

                if limit.is_none() || (a <= limit.unwrap() && b <= limit.unwrap()) {
                    let cost = a * 3 + b;
                    total += cost;
                }
            }
        }
    }
    total as u64
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, machines) = parse_input(input).unwrap();

    let machines = machines
        .into_iter()
        .map(|m| ClawMachine {
            prize_location: Coord {
                x: m.prize_location.x + 10000000000000,
                y: m.prize_location.y + 10000000000000,
            },
            ..m
        })
        .collect_vec();

    Some(calculate_cost(&machines, None))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
