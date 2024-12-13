use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    multi::many1,
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

advent_of_code::solution!(13);

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}
impl Coord {
    fn from_tuple(t: (i32, i32)) -> Coord {
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
                        preceded(tag("X+"), i32),
                        tag(", "),
                        preceded(tag("Y+"), i32),
                    ),
                ),
                newline,
            ),
            terminated(
                preceded(
                    tag("Button B: "),
                    separated_pair(
                        preceded(tag("X+"), i32),
                        tag(", "),
                        preceded(tag("Y+"), i32),
                    ),
                ),
                newline,
            ),
            terminated(
                preceded(
                    tag("Prize: "),
                    separated_pair(
                        preceded(tag("X="), i32),
                        tag(", "),
                        preceded(tag("Y="), i32),
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
        newline,
    ))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, machines) = parse_input(input).unwrap();

    let mut total = 0;
    for machine in machines {
        let mut cheapest = None;
        for a in 0..=100 {
            for b in 0..=100 {
                if machine.a_increment.x * a + machine.b_increment.x * b == machine.prize_location.x
                    && machine.a_increment.y * a + machine.b_increment.y * b
                        == machine.prize_location.y
                {
                    let cost = a * 3 + b;
                    if let Some(c) = cheapest {
                        cheapest = Some(cost.min(c));
                    } else {
                        cheapest = Some(cost);
                    }
                }
            }
        }
        if let Some(c) = cheapest {
            total += c;
        }
    }

    Some(total as u32)
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
