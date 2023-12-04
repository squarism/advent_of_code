use std::collections::HashMap;

use itertools::Itertools;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    let mut first_from_left: u32;
    let mut first_from_right: u32;

    for line in input.lines() {
        first_from_left = first_number(line.to_string());
        first_from_right = first_number(line.to_string().chars().rev().join(""));
        sum += first_from_left * 10 + first_from_right;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut sum: usize = 0;

    for line in input.lines() {
        let left = first_left_number_or_word(line.to_owned());
        if left.is_some() {
            sum += left? * 10;
        }

        let right = first_right_number_or_word(line.to_owned());
        if right.is_some() {
            sum += right?;
        }
    }

    Some(sum)
}

pub fn first_number(string: String) -> u32 {
    let chars = string.chars();
    let first = chars.find_or_first(|&e| e.is_ascii_digit());
    first.unwrap().to_digit(10).unwrap()
}

pub fn first_left_number_or_word(string: String) -> Option<usize> {
    for i in 0..string.len() {
        let substring = substring(&string, i, string.len());

        for num in numbers() {
            if substring.starts_with(num) {
                return Some(num.parse::<usize>().unwrap());
            }
        }

        for word in number_words() {
            if substring.starts_with(word) {
                return Some(string_to_number(word));
            }
        }
    }

    None
}

pub fn first_right_number_or_word(string: String) -> Option<usize> {
    for i in 0..string.len() {
        let substring = substring(&string, 0, string.len() - i);

        for num in numbers() {
            if substring.ends_with(num) {
                return Some(num.parse::<usize>().unwrap());
            }
        }

        for word in number_words() {
            if substring.ends_with(word) {
                return Some(string_to_number(word));
            }
        }
    }

    None
}

pub fn number_words() -> [&'static str; 9] {
    [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
}

pub fn numbers() -> [&'static str; 9] {
    ["1", "2", "3", "4", "5", "6", "7", "8", "9"]
}

fn substring(s: &str, start: usize, end: usize) -> String {
    s.chars().skip(start).take(end - start).collect()
}

pub fn string_to_number(s: &str) -> usize {
    let mut replacements = HashMap::new();
    replacements.insert("zero".to_owned(), 0);
    replacements.insert("one".to_owned(), 1);
    replacements.insert("two".to_owned(), 2);
    replacements.insert("three".to_owned(), 3);
    replacements.insert("four".to_owned(), 4);
    replacements.insert("five".to_owned(), 5);
    replacements.insert("six".to_owned(), 6);
    replacements.insert("seven".to_owned(), 7);
    replacements.insert("eight".to_owned(), 8);
    replacements.insert("nine".to_owned(), 9);

    let lookup = replacements.get(s);
    if lookup.is_none() {
        dbg!(s);
    }
    let number: usize = *lookup.unwrap();
    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    // part two's twist from m. night shyamalan!
    // mutating the data to replace words with numbers
    // is bad idea, we only need to find the left most "thing"
    #[test]
    fn test_part_two() {
        let example = "two1nine\neighttwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let result = part_two(example);
        assert_eq!(result, Some(281));
    }

    #[test]
    fn test_first_number() {
        let string: String = "abc4news".to_owned();
        let result = first_number(string);

        assert_eq!(result, 4);
    }

    #[test]
    fn test_first_left_number_or_word() {
        let string = "xtwonex";
        let result = first_left_number_or_word(string.to_string());

        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_finding_a_number_left() {
        let result = first_left_number_or_word("4isgreat".to_string());

        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_first_left_number_or_word_middle() {
        let string = "x7x";
        let result = first_left_number_or_word(string.to_string());

        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_right_number_or_word_end() {
        let string = "abc7";
        let result = first_right_number_or_word(string.to_string());

        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_right_number_or_word_middle() {
        let string = "a8a";
        let result = first_right_number_or_word(string.to_string());

        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_right_word_middle() {
        let string = "xtwonex";
        let result = first_right_number_or_word(string.to_string());

        assert_eq!(result, Some(1));
    }
}
