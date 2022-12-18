#![allow(unused)]

use itertools::Itertools;
use regex::Regex;

#[derive(PartialEq, Eq, Debug)]
struct Command {
    quantity: u32,
    source: usize,
    destination: usize,
}

type Stack = Vec<Vec<Option<char>>>;

pub fn part_one(input: &str) -> Option<String> {
    let (stack_text, command_text) = split_input(input);
    let mut stack = pivot_and_reverse(parse_stacks(stack_text));

    for command in command_text.lines() {
        stack = operate(parse_command(command), stack);
    }

    let mut message = "".to_owned();

    for column in stack.into_iter() {
        let top = column.last().expect("column last blew up");
        message = format!("{}{}", message, top.unwrap());
    }
    Some(message)
}

pub fn part_two(input: &str) -> Option<String> {
    let (stack_text, command_text) = split_input(input);
    let mut stack = pivot_and_reverse(parse_stacks(stack_text));

    for command in command_text.lines() {
        stack = operate9001(parse_command(command), stack);
    }

    // TODO: why isn't there a better way to build a string incrementally?
    // what about collect into a String?
    let mut message = "".to_owned();

    for column in stack.into_iter() {
        let top = column.last().expect("column last blew up");
        message = format!("{}{}", message, top.unwrap());
    }
    Some(message)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn split_input(input: &str) -> (&str, &str) {
    let mut split = input.split("\n\n");
    (split.next().unwrap(), split.next().unwrap())
}

fn parse_stacks(input: &str) -> Stack {
    let mut lines_of_characters: Stack = vec![];
    let number_of_lines = input.lines().count();

    let re = Regex::new(r"\[([[:alpha:]])\]").unwrap();

    for (i, line) in input.lines().enumerate() {
        // skip the last line of metadata
        if i >= number_of_lines - 1 {
            continue;
        }

        let mut row: Vec<Option<char>> = vec![];
        for group in &line.chars().chunks(4) {
            let cell = group.collect::<String>();
            let cell = cell.trim().to_owned();

            let m = if cell.is_empty() {
                None
            } else {
                let captures = re.captures(&cell);
                let character: char = match captures {
                    Some(capture) => capture.get(1).unwrap().as_str().parse().unwrap(),
                    None => panic!("Unexpected input"),
                };
                Some(character.to_owned())
            };

            row.push(m);
        }
        lines_of_characters.push(row);
    }
    lines_of_characters
}

fn parse_command(input: &str) -> Command {
    let re = Regex::new(r"^move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)").unwrap();
    let captures = re.captures(input).unwrap();

    let quantity = captures.get(1).unwrap().as_str().parse().unwrap();
    let source = captures.get(2).unwrap().as_str().parse().unwrap();
    let destination = captures.get(3).unwrap().as_str().parse().unwrap();

    Command {
        quantity,
        source,
        destination,
    }
}

fn operate(command: Command, stack: Stack) -> Stack {
    let mut mutated = stack.clone();

    for i in (0..command.quantity) {
        let krate = &mut mutated[command.source - 1].pop().unwrap();

        if krate.is_some() {
            mutated[command.destination - 1].push(*krate);
        }
    }

    mutated
}

// "The CrateMover 9001 has the ability to pick up and move multiple crates at once"
fn operate9001(command: Command, stack: Stack) -> Stack {
    let mut mutated = stack.clone();

    // the MAW of the CLAW!  rawwwrrrr!!!
    let mut maw: Vec<Option<char>> = vec![];

    for i in (0..command.quantity) {
        let krate = &mut mutated[command.source - 1].pop().unwrap();

        if krate.is_some() {
            maw.push(*krate);
        }
    }

    for i in (0..maw.len()) {
        let krate = &mut maw.pop().unwrap();
        mutated[command.destination - 1].push(*krate);
    }

    mutated
}

fn pivot_and_reverse(source: Stack) -> Stack {
    let num_rows = source.len();
    let num_cols = source[num_rows - 1].len();

    let mut stack: Stack = vec![];
    for column_number in (0..num_cols).rev() {
        let mut new_row: Vec<Option<char>> = Vec::new();

        source.iter().rev().for_each(|row| {
            let cell = row[column_number];
            if cell.is_some() {
                new_row.push(cell);
            }
        });
        stack.push(new_row);
    }
    stack.reverse();
    stack
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        let result = part_one(&input);

        let expected = "CMZ".to_owned();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        let result = part_two(&input);

        let expected = "MCD".to_owned();
        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_split_input() {
        let input = advent_of_code::read_file("examples", 5);
        let result = split_input(&input);

        let expected = (
            "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ",
            "move 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n",
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_stacks() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ";
        let result = parse_stacks(input);

        let expected = vec![
            vec![None, Some('D'), None],
            vec![Some('N'), Some('C'), None],
            vec![Some('Z'), Some('M'), Some('P')],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_command() {
        let input = "move 1 from 2 to 1";
        let result = parse_command(input);

        let expected = Command {
            quantity: 1,
            source: 2,
            destination: 1,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pivot() {
        let source = vec![
            vec![Some('A'), Some('B'), None],
            vec![Some('D'), Some('E'), Some('F')],
            vec![Some('G'), Some('H'), Some('I')],
        ];

        let result = pivot_and_reverse(source);

        let expected = vec![
            vec![Some('G'), Some('D'), Some('A')],
            vec![Some('H'), Some('E'), Some('B')],
            vec![Some('I'), Some('F')],
        ];
        assert_eq!(result, expected);
    }
}
