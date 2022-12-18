#![allow(unused)]

use std::ops::{Range, RangeInclusive};

#[derive(PartialEq, Eq, Debug)]
pub struct Assignment {
    first: RangeInclusive<u32>,
    second: RangeInclusive<u32>,
}

pub fn parse_line(line: &str) -> Assignment {
    let mut split = line.split(',');
    let assignments = (split.next().unwrap(), split.next().unwrap());

    let mut first_split = assignments.0.split('-');
    let mut second_split = assignments.1.split('-');

    let first = (first_split.next().unwrap(), first_split.next().unwrap());
    let second = (second_split.next().unwrap(), second_split.next().unwrap());

    let first_start = first.0.parse().unwrap();
    let first_end = first.1.parse().unwrap();
    let second_start = second.0.parse().unwrap();
    let second_end = second.1.parse().unwrap();

    Assignment {
        first: RangeInclusive::new(first_start, first_end),
        second: RangeInclusive::new(second_start, second_end),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut overlap_count = 0;

    for line in input.lines() {
        let assignment = parse_line(line);

        let mut first = assignment.first;
        let mut second = assignment.second;

        // check if second is inside first
        let first_contains_second = first.contains(second.start()) && first.contains(second.end());

        // check if second is inside first
        let second_contains_first = second.contains(first.start()) && second.contains(first.end());

        // but maybe they are both true!  :O
        if first_contains_second || second_contains_first {
            overlap_count += 1;
        }
    }

    Some(overlap_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    // #[test]
    // fn test_part_two() {
    //     let input = advent_of_code::read_file("examples", 4);
    //     assert_eq!(part_two(&input), None);
    // }

    #[test]
    fn test_parse_line() {
        let line = "2-4,6-8";
        let result = parse_line(line);

        let expected = Assignment {
            first: RangeInclusive::new(2, 4),
            second: RangeInclusive::new(6, 8),
        };
        assert_eq!(result, expected);
    }
}
