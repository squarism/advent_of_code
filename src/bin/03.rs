#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use itertools::Itertools;

struct Rucksack {
    left: String,
    right: String,
    left_characters: Vec<char>,
    right_characters: Vec<char>,
}

impl Rucksack {
    fn unique_characters(&mut self) {
        self.left_characters = self.left.chars().unique().collect();
        self.right_characters = self.right.chars().unique().collect();
    }

    fn find_common_element(self) -> Option<char> {
        for c in self.left_characters.iter() {
            if self.right_characters.contains(c) {
                return Some(*c);
            }
        }
        None
    }
}

fn split_line(line: &str) -> Rucksack {
    let midpoint = line.len() / 2;
    let left = line.to_string()[0..midpoint].to_owned();
    let right = line.to_string()[midpoint..line.len()].to_owned();

    Rucksack {
        left,
        right,
        left_characters: vec![],
        right_characters: vec![],
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rucksack;
    let mut score = 0;

    for line in input.lines() {
        // println!("line: {}", line);

        rucksack = split_line(line);
        rucksack.unique_characters();

        let common = rucksack.find_common_element();

        match common {
            Some(c) => {
                // println!("The common element is {}", c);
                score += match c {
                    'a'..='z' => c as u32 - 'a' as u32 + 1,
                    'A'..='Z' => c as u32 - 'A' as u32 + 27,
                    _ => 0,
                };
            }
            None => {
                println!("There is no common element")
            }
        }
    }

    Some(score)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    // #[test]
    // fn test_part_two() {
    //     let input = advent_of_code::read_file("examples", 3);
    //     assert_eq!(part_two(&input), None);
    // }
}
