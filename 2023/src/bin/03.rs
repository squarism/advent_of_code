advent_of_code::solution!(3);

use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

// a string of numbers on a grid
#[derive(Clone, Debug, PartialEq)]
pub struct NumberString {
    value: usize,
    positions: Vec<Point>,
}

pub fn part_one(input: &str) -> Option<usize> {
    let number_positions = number_positions(input);
    let mut hits: Vec<NumberString> = vec![];

    for span in number_positions {
        if has_symbol_neighbor(input, span.clone()) {
            hits.push(span);
        }
    }

    Some(hits.iter().fold(0, |acc, n| acc + n.value))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

pub fn dimensions(grid: &str) -> (usize, usize) {
    let mut split = grid.split('\n');
    let width = split.next().unwrap().len();
    let height = split.collect_vec().len();

    (width, height)
}

pub fn number_positions(grid: &str) -> Vec<NumberString> {
    let mut number_positions = vec![];
    let re = Regex::new(r"\d+").unwrap();

    for (i, line) in grid.lines().enumerate() {
        for hit in re.find_iter(line) {
            let match_str = hit.as_str();
            let start = hit.start();
            let end = hit.end();
            let positions: Vec<Point> = (start..end).map(|x| Point { x, y: i }).collect();

            number_positions.push(NumberString {
                value: match_str.parse::<usize>().unwrap(),
                positions,
            })
        }
    }

    number_positions
}

pub fn has_symbol_neighbor(grid: &str, span: NumberString) -> bool {
    // this makes a grid like [y][x] for some reason
    let split_grid: Vec<Vec<char>> = grid.lines().map(|line| line.chars().collect()).collect();
    let dimensions = dimensions(grid);

    // the naive way of not removing self points
    for point in span.positions {
        // search W
        if point.x > 0 && is_symbol(&(split_grid[point.y][point.x - 1])) {
            return true;
        }

        // search NW
        if point.x > 0 && point.y != 0 && is_symbol(&(split_grid[point.y - 1][point.x - 1])) {
            return true;
        }

        // search N
        if point.y > 0 && is_symbol(&(split_grid[point.y - 1][point.x])) {
            return true;
        }

        // search NE
        if point.x < dimensions.0 - 1
            && point.y > 0
            && is_symbol(&(split_grid[point.y - 1][point.x + 1]))
        {
            return true;
        }

        // search E
        if point.x < dimensions.0 - 1 && is_symbol(&(split_grid[point.y][point.x + 1])) {
            return true;
        }

        // search SE
        if point.x < dimensions.0 - 1
            && point.y < dimensions.1 - 1
            && is_symbol(&(split_grid[point.y + 1][point.x + 1]))
        {
            return true;
        }

        // search S
        if point.y < dimensions.1 - 1 && is_symbol(&(split_grid[point.y + 1][point.x])) {
            return true;
        }

        // search SW
        if point.x > 0
            && point.y < dimensions.1 - 1
            && is_symbol(&(split_grid[point.y + 1][point.x - 1]))
        {
            return true;
        }
    }
    false
}

pub fn is_symbol(character: &char) -> bool {
    // . and digits are not symbols, normal characters
    let mut normals: Vec<char> = (0..10)
        .map(|n| std::char::from_digit(n, 10).unwrap())
        .collect();
    normals.push('.');

    // if it's not in this list, it must be a symbol
    !normals.contains(character)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_dimensions() {
        let grid = indoc! {"
            .....
            .....
        "};
        let result = dimensions(grid);

        assert_eq!(result, (5, 2));
    }

    #[test]
    fn test_number_positions() {
        let grid = indoc! {"
            .123...
            .......
            ...777.
        "};
        let result = number_positions(grid);

        let expected: Vec<NumberString> = vec![
            NumberString {
                value: 123,
                positions: vec![
                    Point { x: 1, y: 0 },
                    Point { x: 2, y: 0 },
                    Point { x: 3, y: 0 },
                ],
            },
            NumberString {
                value: 777,
                positions: vec![
                    Point { x: 3, y: 2 },
                    Point { x: 4, y: 2 },
                    Point { x: 5, y: 2 },
                ],
            },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_symbol() {
        assert!(!is_symbol(&'.'));
        assert!(!is_symbol(&'4'));
        assert!(is_symbol(&'!'));
    }

    #[test]
    fn test_search_hit_internal() {
        // This grid was mutated in-place to avoid many DRY tests.  Real project, would keep them.
        let grid = indoc! {"
            .......
            .456...
            *......
        "};
        let span = NumberString {
            value: 456,
            positions: vec![
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
                Point { x: 3, y: 1 },
            ],
        };

        let result = has_symbol_neighbor(grid, span);
        assert!(result);
    }

    #[test]
    fn test_search_edge() {
        let grid = indoc! {"
            .123...
            ....!..
            ...777.
        "};
        let span = NumberString {
            value: 123,
            positions: vec![
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 },
            ],
        };

        let result = has_symbol_neighbor(grid, span);
        assert!(result);
    }
}
