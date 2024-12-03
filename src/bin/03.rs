use nom::{
    bytes::complete::tag,
    character::complete::u32,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

advent_of_code::solution!(3);

#[derive(Debug)]
enum Instruction {
    DO,
    DONT,
    MUL,
}

#[derive(Debug)]
struct Command {
    instruction: Instruction,
    arguments: (u32, u32),
}

fn parse_input(input: &str) -> Vec<Command> {
    fn parse_mul_command(cmd: &str) -> IResult<&str, (u32, u32)> {
        preceded(
            tag("mul"),
            delimited(tag("("), separated_pair(u32, tag(","), u32), tag(")")),
        )(cmd)
    }

    let mut commands = vec![];

    let mut remaining = input;
    while !remaining.is_empty() {
        if remaining.starts_with("do()") {
            remaining = &remaining[4..];
            commands.push(Command {
                instruction: Instruction::DO,
                arguments: (0, 0),
            });
        } else if remaining.starts_with("don't()") {
            remaining = &remaining[6..];
            commands.push(Command {
                instruction: Instruction::DONT,
                arguments: (0, 0),
            });
        } else if let Ok((rest, arguments)) = parse_mul_command(remaining) {
            remaining = rest;
            commands.push(Command {
                instruction: Instruction::MUL,
                arguments,
            });
        } else {
            remaining = &remaining[1..]
        }
    }

    commands

    // many1(preceded(
    //     opt(char),
    //     alt((
    //         pair(tag("do"), tag("()")),
    //         pair(tag("don't"), tag("()")),
    //         preceded(
    //             tag("mul"),
    //             delimited(tag("("), separated_pair(digit1, tag(","), digit1), tag(")")),
    //         ),
    //     )),
    // ))(input)
}

fn run(commands: &Vec<Command>) -> u32 {
    let mut total = 0;
    let mut enabled = true;
    for cmd in commands {
        match cmd.instruction {
            Instruction::DO => {
                enabled = true;
            }
            Instruction::DONT => {
                enabled = false;
            }
            Instruction::MUL => {
                if enabled {
                    total += cmd.arguments.0 * cmd.arguments.1;
                }
            }
        }
    }
    total
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(run(&parse_input(input)))
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
