use itertools::Itertools;
use nom::{
    character::complete::{newline, one_of},
    combinator::opt,
    multi::{many1, many_m_n},
    sequence::terminated,
    IResult,
};

advent_of_code::solution!(25);

#[derive(Debug, PartialEq)]
enum SchematicType {
    Key,
    Lock,
}

#[derive(Debug)]
struct Schematic {
    schematic_type: SchematicType,
    heights: Vec<usize>,
    available_space: usize,
}

impl Schematic {
    fn from(s: &Vec<Vec<char>>) -> Self {
        let w = s[0].len();
        let schematic_type = if s[0].iter().all(|&v| v == '#') {
            SchematicType::Lock
        } else {
            SchematicType::Key
        };

        let mut heights = vec![];
        for x in 0..w {
            heights.push(s.iter().filter(|row| row[x] == '#').count() - 1);
        }

        Schematic {
            schematic_type,
            heights,
            available_space: s.len() - 2,
        }
    }

    fn fits(&self, k: &Schematic) -> bool {
        assert_eq!(self.schematic_type, SchematicType::Lock);
        self.heights
            .iter()
            .enumerate()
            .all(|(i, h)| self.available_space - h >= k.heights[i])
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Vec<char>>>> {
    many1(terminated(
        many_m_n(7, 7, terminated(many1(one_of(".#")), newline)),
        opt(newline),
    ))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, schematics) = parse_input(input).unwrap();

    let schematics = schematics.iter().map(|s| Schematic::from(s)).collect_vec();

    let keys = schematics
        .iter()
        .filter(|s| s.schematic_type == SchematicType::Key)
        .collect_vec();
    let locks = schematics
        .iter()
        .filter(|s| s.schematic_type == SchematicType::Lock)
        .collect_vec();

    let mut fit = 0;
    for k in keys {
        for l in &locks {
            if l.fits(k) {
                fit += 1;
            }
        }
    }

    Some(fit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
