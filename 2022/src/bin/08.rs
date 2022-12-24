#![allow(unused)]

use itertools::assert_equal;
use std::f64;

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = grid_it(input);
    let edge = calculate_edge(grid.clone());

    let mut count = 0;
    // loop through the middle of the grid and find if visible
    for y in (1..grid.length - 1) {
        for x in (1..grid.length - 1) {
            let index = y * grid.length + x;
            let source_tree = grid.items[index];

            let paths = walk_to_edges(grid.clone(), x, y);
            let visible = is_visible(source_tree, paths);
            if visible {
                count += 1
            }
        }
    }

    Some(edge + count)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    fn input() -> &'static str {
        indoc! {"
        30373
        25512
        65332
        33549
        35390
        "}
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_grid_it() {
        let mut result = grid_it(input());

        let expected = 25;
        assert_eq!(result.items().len(), expected);
    }

    #[test]
    fn test_calculate_edge() {
        let mut grid = grid_it(input());
        let result = calculate_edge(grid);

        let expected = 16;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_walk_to_edges() {
        let mut grid = grid_it(input());
        let result = walk_to_edges(grid, 1, 1);

        let expected = vec![vec![0], vec![5, 1, 2], vec![5, 3, 5], vec![2]];
        assert_equal(result.iter(), expected.iter());
    }

    #[test]
    fn test_is_visible() {
        let source_tree = 5;
        let paths_to_edges = vec![vec![0], vec![5, 1, 2], vec![5, 3, 5], vec![2]];

        let result = is_visible(source_tree, paths_to_edges);

        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_hidden() {
        let source_tree = 5;
        let paths_to_edges = vec![vec![5], vec![6, 1, 2], vec![5, 3, 5], vec![9]];

        let result = is_visible(source_tree, paths_to_edges);

        let expected = false;
        assert_eq!(result, expected);
    }
}

#[derive(Debug, Clone)]
struct Grid {
    items: Vec<usize>,
    length: usize,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            items: vec![],
            length: 0,
        }
    }

    pub fn push_many<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = usize>,
    {
        self.items.extend(items);
    }

    pub fn push(&mut self, item: usize) {
        self.items.push(item);
    }

    pub fn items(&mut self) -> Vec<usize> {
        self.items.clone()
    }

    pub fn set_length(&mut self, length: usize) {
        self.length = length;
    }
}

fn grid_it(input: &str) -> Grid {
    let mut grid = Grid::new();

    let mut count = 0;
    for line in input.lines() {
        let mut items = grid.items();
        let digits = line
            .chars()
            .map(|c| usize::try_from(c.to_digit(10).unwrap()).unwrap());
        grid.push_many(digits);
        count += 1
    }

    grid.set_length(count);
    grid
}

// the grid has to be square
fn calculate_edge(grid: Grid) -> usize {
    grid.length + ((grid.length - 1) * 2) + (grid.length - 2)
}

fn walk_to_edges(grid: Grid, x: usize, y: usize) -> Vec<Vec<usize>> {
    let length = grid.length;
    let origin = length * y + x;

    let north: Vec<usize> = (0..y)
        .rev()
        .map(|i| {
            let index = length * i + x;
            grid.items[index]
        })
        .collect();

    let east: Vec<usize> = (x + 1..length)
        .map(|i| {
            let index = length * y + i;
            grid.items[index]
        })
        .collect();

    let south: Vec<usize> = (y + 1..length)
        .map(|i| {
            let index = length * i + x;
            grid.items[index]
        })
        .collect();

    let west: Vec<usize> = (0..x)
        .rev()
        .map(|i| {
            let index = length * y + i;
            grid.items[index]
        })
        .collect();

    vec![north, east, south, west]
}

fn is_visible(source_tree: usize, paths_to_edges: Vec<Vec<usize>>) -> bool {
    for path in paths_to_edges {
        if path.iter().all(|tree| tree < &source_tree) {
            return true;
        }
    }

    false
}
