#[derive(Debug)]
enum Throw {
    Win,
    Lose,
    Draw,
}

struct Move {
    them: String,
    you: String,
}

fn judge(you: &str, opponent: &str) -> Throw {
    match (you, opponent) {
        ("X", "A") => Throw::Draw,
        ("X", "B") => Throw::Lose,
        ("X", "C") => Throw::Win,
        ("Y", "B") => Throw::Draw,
        ("Y", "C") => Throw::Lose,
        ("Y", "A") => Throw::Win,
        ("Z", "C") => Throw::Draw,
        ("Z", "A") => Throw::Lose,
        ("Z", "B") => Throw::Win,
        _ => panic!(),
    }
}

fn fake_it<'a>(you: &'a str, opponent: &'a str) -> &'a str {
    match (you, opponent) {
        ("X", "A") => "Z",
        ("X", "B") => "X",
        ("X", "C") => "Y",
        ("Y", "A") => "X",
        ("Y", "B") => "Y",
        ("Y", "C") => "Z",
        ("Z", "A") => "Y",
        ("Z", "B") => "Z",
        ("Z", "C") => "X",
        _ => panic!(),
    }
}

fn thrown_bonus(you: &str) -> u32 {
    match you {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!(),
    }
}

fn parse_moves(input: &str) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();

        let mov: Move = Move {
            them: split.next().unwrap().to_owned(),
            you: split.next().unwrap().to_owned(),
        };

        moves.push(mov);
    }
    moves
}

pub fn part_one(input: &str) -> Option<u32> {
    let moves = parse_moves(input);

    let mut total_score = 0;
    for mov in moves {
        let event = judge(mov.you.as_str(), mov.them.as_str());
        let score = match event {
            Throw::Lose => 0,
            Throw::Draw => 3,
            Throw::Win => 6,
        };
        total_score += score;
        total_score += thrown_bonus(mov.you.as_str());
    }

    Some(total_score)
}

// X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win.
pub fn part_two(input: &str) -> Option<u32> {
    let moves = parse_moves(input);

    let mut total_score = 0;
    for mov in moves {
        let fake = fake_it(mov.you.as_str(), mov.them.as_str());
        let event = judge(fake, mov.them.as_str());
        let score = match event {
            Throw::Lose => 0,
            Throw::Draw => 3,
            Throw::Win => 6,
        };
        total_score += score;
        total_score += thrown_bonus(fake);
    }
    Some(total_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
