use itertools::Itertools;
use nom::{character::complete::digit1, IResult, InputIter};

advent_of_code::solution!(9);

struct FileRecord {
    pos: usize,
    size: usize,
}

fn parse_input(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

fn create_disk_map(input: &str) -> Vec<usize> {
    let (_, disk_map) = parse_input(input).unwrap();

    disk_map
        .iter_elements()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec()
}

fn create_disk(disk_map: Vec<usize>) -> (Vec<i32>, Vec<FileRecord>) {
    let disk_size: usize = disk_map.iter().sum();
    let mut disk = Vec::with_capacity(disk_size);

    let mut blocks = vec![];
    for (i, &size) in disk_map.iter().enumerate() {
        if i % 2 == 0 {
            let id = i / 2;
            let pos = disk.len();
            blocks.push(FileRecord { pos, size });
            for _ in 0..size {
                disk.push(id as i32);
            }
        } else {
            for _ in 0..size {
                disk.push(-1);
            }
        }
    }
    assert_eq!(disk.len(), disk_size);

    (disk, blocks)
}

fn checksum_disk(disk: Vec<i32>) -> u64 {
    disk.iter()
        .enumerate()
        .filter(|(_, &v)| v != -1)
        .fold(0, |acc, (pos, &v)| acc + (pos as u64 * v as u64))
}

pub fn part_one(input: &str) -> Option<u64> {
    let disk_map = create_disk_map(input);
    let (mut disk, _) = create_disk(disk_map);

    let mut last_ptr = disk.len() - 1;
    for i in 0..disk.len() {
        if i >= last_ptr {
            break;
        }
        if disk[i] == -1 {
            disk[i] = disk[last_ptr];
            disk[last_ptr] = -1;
            while disk[last_ptr] == -1 {
                last_ptr -= 1;
            }
        }
    }

    Some(checksum_disk(disk))
}

pub fn part_two(input: &str) -> Option<u64> {
    let disk_map = create_disk_map(input);
    let (mut disk, blocks) = create_disk(disk_map);

    for (id, rec) in blocks.iter().enumerate().rev() {
        if let Some(dest) = disk.iter().enumerate().position(|(i, &v)| {
            v == -1
                && i < rec.pos
                && i + rec.size <= disk.len()
                && disk[i..i + rec.size].iter().all(|&v| v == -1)
        }) {
            for i in 0..rec.size {
                disk[i + dest] = id as i32;
                disk[i + rec.pos] = -1;
            }
        }
    }

    Some(checksum_disk(disk))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
