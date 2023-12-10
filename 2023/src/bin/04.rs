advent_of_code::solution!(4);

use regex::Regex;
use std::{collections::VecDeque, fmt};

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

#[derive(Clone, PartialEq)]
pub struct CardScore {
    id: usize,
    score: usize,
}

impl fmt::Debug for CardScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ id:{} | score:{:?} ]", self.id, self.score)
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

pub fn part_two(input: &str) -> Option<usize> {
    let mut cards: Vec<Card> = vec![];

    for line in input.lines() {
        cards.push(parse_card(line));
    }

    let copies = copy_cards(seed_copies(cards));

    Some(copies.len())
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
    let hits = matching_hits(card);

    // to avoid raising the score to ^0
    if hits == 0 {
        return 0;
    }

    let power: u32 = (hits - 1).try_into().unwrap();
    usize::pow(2, power)
}

// what numbers match between winners and choices on a card
pub fn matching_hits(card: Card) -> usize {
    let hits: Vec<usize> = card
        .choices
        .iter()
        .filter(|choice| card.winners.contains(choice))
        .cloned()
        .collect();
    hits.len()
}

pub fn seed_copies(cards: Vec<Card>) -> Vec<CardScore> {
    let mut scores: Vec<CardScore> = vec![];
    for card in cards {
        scores.push(CardScore {
            id: card.id,
            score: matching_hits(card),
        })
    }
    scores
}

pub fn copy_cards(cards: Vec<CardScore>) -> Vec<CardScore> {
    let mut copies = cards.clone();

    // avoiding recursion?
    let mut stack: VecDeque<CardScore> = cards.clone().into();

    while let Some(card) = stack.pop_front() {
        if card.score == 0 {
            continue;
        }

        let clamp = if stack.len() > 1 {
            usize::min(cards.len() - card.id, card.score)
        } else {
            0
        };

        let copy_list: Vec<CardScore> = cards[(card.id)..(card.id + clamp)].to_vec();

        for copy in copy_list.clone() {
            stack.push_back(copy.clone());
            copies.push(copy.clone());
        }
    }

    copies
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
        assert_eq!(result, Some(30));
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

    #[test]
    fn test_matching_hits() {
        let card = Card {
            id: 42,
            winners: vec![1, 2, 3, 4],
            choices: vec![2, 3],
        };
        let result = matching_hits(card);

        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_seed_copies() {
        let card1 = Card {
            id: 1,
            winners: vec![1, 2, 3, 4, 5],
            choices: vec![2, 3, 4, 5],
        };
        let card2 = Card {
            id: 2,
            winners: vec![1, 2, 3, 4],
            choices: vec![2, 3],
        };
        let cards = vec![card1, card2];
        let result = seed_copies(cards);

        let expected: Vec<CardScore> =
            vec![CardScore { id: 1, score: 4 }, CardScore { id: 2, score: 2 }];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_copy_cards_simple() {
        let seed: Vec<CardScore> = vec![
            CardScore { id: 1, score: 2 },
            CardScore { id: 2, score: 0 },
            CardScore { id: 3, score: 0 },
        ];
        let result = copy_cards(seed);

        let expected: Vec<CardScore> = vec![
            CardScore { id: 1, score: 2 },
            CardScore { id: 2, score: 0 },
            CardScore { id: 3, score: 0 },
            CardScore { id: 2, score: 0 },
            CardScore { id: 3, score: 0 },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_copy_cards_medium() {
        let seed: Vec<CardScore> = vec![
            CardScore { id: 1, score: 1 },
            CardScore { id: 2, score: 1 },
            CardScore { id: 3, score: 1 },
        ];
        let result = copy_cards(seed);

        let expected: Vec<CardScore> = vec![
            CardScore { id: 1, score: 1 },
            CardScore { id: 2, score: 1 },
            CardScore { id: 3, score: 1 },
            CardScore { id: 2, score: 1 },
            CardScore { id: 3, score: 1 },
        ];
        assert_eq!(result, expected);
    }
}
