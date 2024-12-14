use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::opt,
    multi::many1,
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};

advent_of_code::solution!(14);

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    position: Coord,
    velocity: Coord,
}
impl Robot {
    fn update_position(&mut self, w: i32, h: i32) {
        self.position.x = (self.position.x + self.velocity.x).rem_euclid(w);
        self.position.y = (self.position.y + self.velocity.y).rem_euclid(h);
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Robot>> {
    many1(
        terminated(
            separated_pair(
                preceded(tag("p="), separated_pair(i32, tag(","), i32)),
                tag(" "),
                preceded(tag("v="), separated_pair(i32, tag(","), i32)),
            ),
            opt(newline),
        )
        .map(|(p, v)| Robot {
            position: Coord { x: p.0, y: p.1 },
            velocity: Coord { x: v.0, y: v.1 },
        }),
    )(input)
}

fn part_one_process(input: &str, width: i32, height: i32) -> Option<u32> {
    let (_, mut positions) = parse_input(input).unwrap();

    for _ in 0..100 {
        for p in positions.iter_mut() {
            p.update_position(width, height);
        }
    }

    let middle_h = width / 2;
    let middle_v = height / 2;

    let mut count = [0u32; 4];
    for p in positions {
        if p.position.y < middle_v && p.position.x < middle_h {
            count[0] += 1;
        } else if p.position.y < middle_v && p.position.x > middle_h {
            count[1] += 1;
        } else if p.position.y > middle_v && p.position.x < middle_h {
            count[2] += 1;
        } else if p.position.y > middle_v && p.position.x > middle_h {
            count[3] += 1;
        }
    }

    count.into_iter().reduce(|acc, e| acc * e)
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one_process(input, 101, 103)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, mut positions) = parse_input(input).unwrap();

    let expected = [[' '; 16], ['#'; 16]].concat();

    for i in 0..10000 {
        let mut grid = [[' '; 101]; 103];
        for p in positions.iter_mut() {
            p.update_position(101, 103);
            grid[p.position.y as usize][p.position.x as usize] = '#';
        }

        if grid[49].starts_with(&expected) {
            return Some(i + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_process(&advent_of_code::template::read_file("examples", DAY), 11, 7);
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
