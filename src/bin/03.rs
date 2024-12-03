use nom::{
    bytes::complete::tag,
    character::complete::u32,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

advent_of_code::solution!(3);

#[derive(Debug)]
enum Command {
    Do(),
    Dont(),
    Mul(u32, u32),
}

impl Command {
    fn parse(input: &str) -> Option<(&str, Self)> {
        fn parse_no_args<'a>(name: &'a str, cmd: &'a str) -> IResult<&'a str, &'a str> {
            preceded(tag(name), delimited(tag("("), tag(""), tag(")")))(cmd)
        }
        fn parse_mul(cmd: &str) -> IResult<&str, (u32, u32)> {
            preceded(
                tag("mul"),
                delimited(tag("("), separated_pair(u32, tag(","), u32), tag(")")),
            )(cmd)
        }
        if let Ok((rest, _)) = parse_no_args("do", input) {
            Some((rest, Command::Do()))
        } else if let Ok((rest, _)) = parse_no_args("don't", input) {
            Some((rest, Command::Dont()))
        } else if let Ok((rest, args)) = parse_mul(input) {
            Some((rest, Command::Mul(args.0, args.1)))
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> Vec<Command> {
    let mut commands = vec![];

    let mut remaining = input;
    while !remaining.is_empty() {
        if let Some((rest, cmd)) = Command::parse(remaining) {
            remaining = rest;
            commands.push(cmd);
        } else {
            remaining = &remaining[1..];
        }
    }

    commands
}

fn run(commands: &Vec<Command>) -> u32 {
    let mut total = 0;
    let mut enabled = true;
    for cmd in commands {
        match cmd {
            Command::Do() => {
                enabled = true;
            }
            Command::Dont() => {
                enabled = false;
            }
            Command::Mul(v1, v2) => {
                if enabled {
                    total += v1 * v2;
                }
            }
        }
    }
    total
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut commands = parse_input(input);
    commands.retain(|c| match c {
        Command::Mul(_, _) => true,
        _ => false,
    });

    Some(run(&commands))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(run(&parse_input(input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
