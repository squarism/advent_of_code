advent_of_code::solution!(2);

use regex::Regex;

#[derive(Copy, Clone, Debug, PartialEq)]
struct CubeSet {
    red: Option<u8>,
    green: Option<u8>,
    blue: Option<u8>,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut sum = 0;

    let re = Regex::new(r"^Game\s(\d+):\s(.*)").unwrap();

    for line in input.lines() {
        let game_id = get_game_id(line).unwrap();

        let sets = re.captures(line).unwrap().get(2).map_or("", |m| m.as_str());
        let sets = parse_game_sets(sets);

        let under_limit = sets
            .iter()
            .all(|set| set.red <= Some(12) && set.green <= Some(13) && set.blue <= Some(14));

        if under_limit {
            sum += game_id
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut sum: usize = 0;

    let re = Regex::new(r"^Game\s(\d+):\s(.*)").unwrap();

    for line in input.lines() {
        let sets = re.captures(line).unwrap().get(2).map_or("", |m| m.as_str());
        let sets = parse_game_sets(sets);

        let max_red = sets.iter().filter_map(|c| c.red).max().map(|v| v as usize);
        let max_green = sets
            .iter()
            .filter_map(|c| c.green)
            .max()
            .map(|v| v as usize);
        let max_blue = sets.iter().filter_map(|c| c.blue).max().map(|v| v as usize);

        let power = max_red? * max_green? * max_blue?;
        sum += power;
    }

    Some(sum)
}

pub fn get_game_id(line: &str) -> Option<usize> {
    let re = Regex::new(r"^Game\s(\d+):\s(.*)").unwrap();
    let Some(groups) = re.captures(line) else {
        return None;
    };
    let game_id: &str = groups.get(1).map_or("", |m| m.as_str());
    Some(game_id.parse::<usize>().unwrap())
}

fn parse_game_sets(input: &str) -> Vec<CubeSet> {
    let mut game_sets: Vec<CubeSet> = vec![];

    let games: Vec<&str> = input.split(';').map(|part| part.trim()).collect();
    for game in games.iter() {
        let mut cube_set = CubeSet {
            red: None,
            green: None,
            blue: None,
        };

        cube_set.blue = parse_number_of_cubes(game, "blue");
        cube_set.green = parse_number_of_cubes(game, "green");
        cube_set.red = parse_number_of_cubes(game, "red");

        game_sets.push(cube_set);
    }

    game_sets
}

// man, I need to learn nom+nom-supreme or chumsky
pub fn parse_number_of_cubes(string: &str, color: &str) -> Option<u8> {
    let pattern = format!(r"(\d+)\s{}", color);
    let re = Regex::new(&pattern).unwrap();

    if !re.is_match(string) {
        return None;
    }

    re.captures(string)
        .unwrap()
        .get(1)
        .map(|m| m.as_str().parse::<u8>().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }

    #[test]
    fn test_get_game_id() {
        let line = "Game 42: blahblah";
        let result = get_game_id(line);

        assert_eq!(result, Some(42))
    }

    #[test]
    fn test_parse_number_of_cubes() {
        let string = "1 blue, 2 green";
        let result = parse_number_of_cubes(string, "green");

        assert_eq!(result, Some(2))
    }

    #[test]
    fn test_parse_game_sets() {
        let input = "1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let result = parse_game_sets(input);

        let expected = [
            CubeSet {
                red: None,
                green: Some(2),
                blue: Some(1),
            },
            CubeSet {
                red: Some(1),
                green: Some(3),
                blue: Some(4),
            },
            CubeSet {
                red: None,
                green: Some(1),
                blue: Some(1),
            },
        ];

        assert_eq!(result, expected);
    }
}
