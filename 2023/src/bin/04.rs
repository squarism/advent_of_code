advent_of_code::solution!(4);

use regex::Regex;
use std::fmt;

#[derive(PartialEq)]
pub struct Card {
    id: usize,
    winners: Vec<usize>,
    choices: Vec<usize>,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[ Card {} ] | winners: {:?} | choices: {:?} |",
            self.id, self.winners, self.choices
        )
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut cards: Vec<Card> = vec![];

    for line in input.lines() {
        cards.push(parse_card(line));
    }

    let mut total = 0;
    for card in cards {
        let score = score_card(card);
        total += score;
    }

    Some(total)
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

pub fn parse_card(line: &str) -> Card {
    let re = Regex::new(r"^Card\s+(\d+):(.*)\|(.*)").unwrap();
    let id = re
        .captures(line)
        .unwrap()
        .get(1)
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .unwrap();

    let winners = re
        .captures(line)
        .unwrap()
        .get(2)
        .map(|m| m.as_str())
        .unwrap();

    let choices = re
        .captures(line)
        .unwrap()
        .get(3)
        .map(|m| m.as_str())
        .unwrap();

    let winners = cleanup_numbers(winners);
    let choices = cleanup_numbers(choices);
    Card {
        id,
        winners,
        choices,
    }
}

pub fn cleanup_numbers(garbage: &str) -> Vec<usize> {
    garbage
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

pub fn score_card(card: Card) -> usize {
    let hits: Vec<usize> = card
        .choices
        .iter()
        .filter(|choice| card.winners.contains(choice))
        .cloned()
        .collect();

    if hits.is_empty() {
        return 0;
    }
    let len = hits.len() - 1;

    usize::pow(2, len.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_card() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let result = parse_card(line);

        let expected = Card {
            id: 1,
            winners: vec![41, 48, 83, 86, 17],
            choices: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_cleanup_numbers() {
        let garbage = "  1 2  3 ";
        let result = cleanup_numbers(garbage);

        let expected = vec![1, 2, 3];
        assert_eq!(result, expected);
    }
}
