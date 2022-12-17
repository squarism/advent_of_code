#![allow(dead_code)]
#![allow(unused)]

use itertools::Itertools;
use std::collections::HashSet;

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

fn score_character(character: char) -> u32 {
    match character {
        'a'..='z' => character as u32 - 'a' as u32 + 1,
        'A'..='Z' => character as u32 - 'A' as u32 + 27,
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rucksack;
    let mut score = 0;

    for line in input.lines() {
        rucksack = split_line(line);
        rucksack.unique_characters();

        let common = rucksack.find_common_element();

        match common {
            Some(c) => {
                score += score_character(c);
            }
            None => {
                println!("There is no common element")
            }
        }
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    // group lines into threes
    let groups = input.lines().into_iter().chunks(3);

    // set up a list of matched characters to score later
    let mut to_score = vec![] as Vec<char>;

    for group in &groups {
        // a set of 3 rucksacks
        let set = group.collect::<Vec<&str>>();

        // assemble the different rucksacks
        let first_chars: HashSet<_> = set[0].chars().collect();
        let mut second_chars: HashSet<_> = set[1].chars().collect();
        let mut third_chars: HashSet<_> = set[2].chars().collect();

        // look for the letter in each sack
        'outer: for c in first_chars {
            if second_chars.contains(&c) && third_chars.contains(&c) {
                to_score.push(c);

                // stop if found in other two
                break 'outer;
            }
        }
    }

    // score the list
    let mut score = 0;
    for character in to_score {
        score += score_character(character);
    }

    Some(score)
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

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
