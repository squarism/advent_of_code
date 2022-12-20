#![allow(unused)]

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    for line in input.lines() {
        for i in (0..=line.len() - 4) {
            let sub = &line[i..i + 4];

            let unique = sub.chars().unique();
            let unique_count = unique.into_iter().count();

            if unique_count == 4 {
                return Some(i + 4);
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    for line in input.lines() {
        for i in (0..=line.len() - 14) {
            let sub = &line[i..i + 14];

            let unique = sub.chars().unique();
            let unique_count = unique.into_iter().count();

            if unique_count == 14 {
                return Some(i + 14);
            }
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
