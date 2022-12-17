#[derive(Debug)]
struct Elf {
    calories: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut elves: Vec<Elf> = Vec::new();

    for group in input.split("\n\n") {
        let mut sum: u32 = 0;

        for line in group.lines() {
            let calories: u32 = line.parse().unwrap();
            sum += calories;
        }

        elves.push(Elf { calories: sum });
    }

    elves.sort_by(|a, b| b.calories.cmp(&a.calories));
    Some(elves.first().unwrap().calories)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves: Vec<Elf> = Vec::new();

    for group in input.split("\n\n") {
        let mut sum: u32 = 0;

        for line in group.lines() {
            let calories: u32 = line.parse().unwrap();
            sum += calories;
        }

        elves.push(Elf { calories: sum });
    }

    elves.sort_by(|a, b| b.calories.cmp(&a.calories));
    let first_three_elves = elves.into_iter().take(3);

    let mut second_sum = 0;
    for elf in first_three_elves {
        second_sum += elf.calories;
        // println!("Elf has {} calories", elf.calories);
    }

    // println!("{}", second_sum);
    Some(second_sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
