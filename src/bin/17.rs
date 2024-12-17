use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline, u8},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

advent_of_code::solution!(17);

struct MachineState {
    a: i32,
    b: i32,
    c: i32,
    code: Vec<u8>,
}
impl MachineState {
    fn run(&mut self) -> String {
        let mut ip = 0;
        let mut output = vec![];

        while ip < self.code.len() {
            let cmd = self.code[ip];
            let operand = self.code[ip + 1];
            ip += 2;
            match cmd {
                0 => {
                    // adv
                    self.a = self.perform_dv(operand);
                }
                1 => {
                    // bxl
                    self.b = self.b ^ (operand as i32);
                }
                2 => {
                    // bst
                    self.b = self.get_combo_operand(operand) % 8;
                }
                3 => {
                    // jnz
                    if self.a != 0 {
                        ip = operand as usize;
                    }
                }
                4 => {
                    // bxc
                    self.b = self.b ^ self.c;
                }
                5 => {
                    // out
                    output.push((self.get_combo_operand(operand) % 8).to_string());
                }
                6 => {
                    // bdv
                    self.b = self.perform_dv(operand);
                }
                7 => {
                    // cdv
                    self.c = self.perform_dv(operand);
                }
                _ => todo!("Command {cmd} not implemented"),
            }
        }
        output.join(",")
    }

    // TODO: check type
    fn get_combo_operand(&self, operand: u8) -> i32 {
        match operand {
            0..=3 => operand as i32,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => unreachable!("Invalid operand 7"), // should be error not panic...
            _ => unreachable!("Invalid combo operand {operand}"),
        }
    }

    fn perform_dv(&self, operand: u8) -> i32 {
        self.a / (1 << self.get_combo_operand(operand))
    }
}

fn parse_input(input: &str) -> IResult<&str, ((i32, i32, i32), Vec<u8>)> {
    fn parse_registers(input: &str) -> IResult<&str, (i32, i32, i32)> {
        tuple((
            terminated(preceded(tag("Register A: "), i32), newline),
            terminated(preceded(tag("Register B: "), i32), newline),
            terminated(preceded(tag("Register C: "), i32), newline),
        ))(input)
    }

    fn parse_code(input: &str) -> IResult<&str, Vec<u8>> {
        preceded(tag("Program: "), separated_list1(tag(","), u8))(input)
    }

    separated_pair(parse_registers, newline, parse_code)(input)
}

pub fn part_one(input: &str) -> Option<String> {
    let (_, ((a, b, c), code)) = parse_input(input).unwrap();
    let mut machine_state = MachineState { a, b, c, code };
    Some(machine_state.run())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine() {
        let mut machine_state = MachineState {
            c: 9,
            b: 0,
            a: 0,
            code: vec![2, 6],
        };
        machine_state.run();
        assert_eq!(1, machine_state.b);

        machine_state = MachineState {
            c: 0,
            b: 0,
            a: 10,
            code: vec![5, 0, 5, 1, 5, 4],
        };
        let out = machine_state.run();
        assert_eq!("0,1,2", out);

        machine_state = MachineState {
            c: 0,
            b: 0,
            a: 2024,
            code: vec![0, 1, 5, 4, 3, 0],
        };
        let out = machine_state.run();
        assert_eq!("4,2,5,6,7,7,7,7,3,1,0", out);
        assert_eq!(0, machine_state.a);

        machine_state = MachineState {
            c: 0,
            b: 29,
            a: 0,
            code: vec![1, 7],
        };
        machine_state.run();
        assert_eq!(26, machine_state.b);

        machine_state = MachineState {
            c: 43690,
            b: 2024,
            a: 0,
            code: vec![4, 0],
        };
        machine_state.run();
        assert_eq!(44354, machine_state.b);

        // TODO: check all operations tested
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
