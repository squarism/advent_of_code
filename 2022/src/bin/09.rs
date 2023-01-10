#![allow(unused)]

use std::cmp::Ordering;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    // starting position, head covers tail
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };

    let mut tail_locations: Vec<Point> = vec![tail.clone()];

    for line in input.lines() {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap().to_owned();
        let count: u32 = split.next().unwrap().parse().unwrap();

        for i in 0..count {
            head = move_point(head, &direction);
            tail = tail_target(head.clone(), tail);
            tail_locations.push(tail.clone());
        }
    }

    let count = count_visits(tail_locations);
    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_movement_x() {
        let head = Point { x: 2, y: 0 };
        let tail = Point { x: 0, y: 0 };

        let result = tail_target(head, tail);

        let expected = Point { x: 1, y: 0 };
        assert_eq!(expected, result);
    }

    // 3-----
    // 2--H--
    // 1-----
    // 0--T--
    //  01234
    #[test]
    fn test_movement_y() {
        let head = Point { x: 2, y: 2 };
        let tail = Point { x: 2, y: 0 };

        let result = tail_target(head, tail);

        let expected = Point { x: 2, y: 1 };
        assert_eq!(expected, result);
    }

    // 3-----
    // 2---H-
    // 1-----
    // 0--T--
    //  01234
    #[test]
    fn test_movement_diag() {
        let head = Point { x: 3, y: 2 };
        let tail = Point { x: 2, y: 0 };

        let result = tail_target(head, tail);

        let expected = Point { x: 3, y: 1 };
        assert_eq!(expected, result);
    }

    // 3--H--
    // 2---T-
    // 1-----
    // 0-----
    //  01234
    #[test]
    fn test_movement_noop() {
        let head = Point { x: 2, y: 3 };
        let tail = Point { x: 3, y: 2 };

        let result = tail_target(head, tail);

        let expected = Point { x: 3, y: 2 };
        assert_eq!(expected, result);
    }

    #[test]
    fn test_count_visits() {
        let movements: Vec<Point> = vec![
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 2, y: 1 },
            Point { x: 1, y: 0 },
        ];

        let result = count_visits(movements);

        let expected = 3;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_move_point() {
        let point = Point { x: 1, y: 0 };
        let direction = "R";

        let result = move_point(point, &direction);

        let expected = Point { x: 2, y: 0 };
        assert_eq!(expected, result);
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Rope {
    head: Point,
    tail: Point,
}

fn tail_target(head: Point, tail: Point) -> Point {
    let x_diff = head.x - tail.x;
    let y_diff = head.y - tail.y;

    let x_movement = match x_diff {
        _ if i32::abs(x_diff) == 1 && i32::abs(y_diff) <= 1 => 0,
        _ if x_diff >= 1 => 1,
        _ if x_diff <= -1 => -1,
        _ => 0,
    };

    let y_movement = match y_diff {
        _ if i32::abs(x_diff) <= 1 && i32::abs(y_diff) == 1 => 0,
        _ if y_diff >= 1 => 1,
        _ if y_diff <= -1 => -1,
        _ => 0,
    };

    let x = tail.x + x_movement;
    let y = tail.y + y_movement;

    Point { x, y }
}

fn count_visits(movements: Vec<Point>) -> usize {
    movements.into_iter().sorted().dedup().count()
}

fn move_point(point: Point, direction: &str) -> Point {
    match direction {
        "R" => Point {
            x: point.x + 1,
            y: point.y,
        },
        "L" => Point {
            x: point.x - 1,
            y: point.y,
        },
        "U" => Point {
            x: point.x,
            y: point.y + 1,
        },
        "D" => Point {
            x: point.x,
            y: point.y - 1,
        },
        _ => point,
    }
}
