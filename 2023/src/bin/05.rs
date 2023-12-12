advent_of_code::solution!(5);

use itertools::Itertools;
use regex::Regex;
use std::usize;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RangeMap {
    source: usize,
    destination: usize,
    length: usize,
}

#[derive(Debug, PartialEq)]
pub struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: Vec<RangeMap>,
    soil_to_fertilizer: Vec<RangeMap>,
    fertilizer_to_water: Vec<RangeMap>,
    water_to_light: Vec<RangeMap>,
    light_to_temperature: Vec<RangeMap>,
    temperature_to_humidity: Vec<RangeMap>,
    humidity_to_location: Vec<RangeMap>,
}

pub fn part_one(input: &str) -> Option<isize> {
    let almanac = parse(input);

    let mut locations = vec![];

    for seed in almanac.seeds {
        let seed = seed as isize;
        let soil = lookup(seed, almanac.seed_to_soil.clone());
        let fert = lookup(soil, almanac.soil_to_fertilizer.clone());
        let watr = lookup(fert, almanac.fertilizer_to_water.clone());
        let lght = lookup(watr, almanac.water_to_light.clone());
        let temp = lookup(lght, almanac.light_to_temperature.clone());
        let humi = lookup(temp, almanac.temperature_to_humidity.clone());
        let locn = lookup(humi, almanac.humidity_to_location.clone());

        locations.push(locn);
    }
    locations.sort();
    let min = *locations.first().unwrap();

    Some(min)
}

// this might be the slowest code I have ever written
pub fn part_two(input: &str) -> Option<isize> {
    let almanac = parse(input);

    let seeds_ranges = part_two_seeds(almanac.seeds);

    let mut min = isize::MAX;

    let mut modulo_counter = 1;

    for seeds in seeds_ranges {
        println!("starting seed range: {}", seeds.len());
        for seed in seeds {
            let seed = seed as isize;
            let soil = lookup(seed, almanac.seed_to_soil.clone());
            let fert = lookup(soil, almanac.soil_to_fertilizer.clone());
            let watr = lookup(fert, almanac.fertilizer_to_water.clone());
            let lght = lookup(watr, almanac.water_to_light.clone());
            let temp = lookup(lght, almanac.light_to_temperature.clone());
            let humi = lookup(temp, almanac.temperature_to_humidity.clone());
            let locn = lookup(humi, almanac.humidity_to_location.clone());

            if modulo_counter % 1_000_000 == 0 {
                modulo_counter = 1;
                print!(".");
            } else {
                modulo_counter += 1;
            }

            if locn < min {
                min = locn;
                println!("New min: {}", min);
            }
        }
    }

    Some(min)
}

pub fn find_text(vector: &[&str], text: &str) -> Option<usize> {
    let result = vector.iter().find_position(|&e| *e == text);

    result.map(|m| m.0)
}

pub fn make_range(text: Vec<&str>, start: usize, end: usize) -> Vec<RangeMap> {
    let text_section: Vec<&str> = text.get(start + 1..end - 1).unwrap().to_vec();

    text_section
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .filter(|v: &Vec<usize>| v.len() == 3)
        .map(|array| {
            let (source, destination, length) = (array[1], array[0], array[2]);
            RangeMap {
                source,
                destination,
                length,
            }
        })
        .collect()
}

pub fn lookup(seed: isize, ranges: Vec<RangeMap>) -> isize {
    match ranges
        .iter()
        .find(|rm| (rm.source..rm.source + rm.length).contains(&(seed as usize)))
    {
        Some(range) => {
            let offset: isize = range.destination as isize - range.source as isize;
            (seed + offset).try_into().unwrap()
        }
        None => seed.try_into().unwrap(),
    }
}

// is it parsing if I'm not using a parser?  Is it?  :|
pub fn parse(input: &str) -> Almanac {
    let sections: Vec<&str> = input.split('\n').collect();

    let seed_line = sections.first().unwrap();
    let re = Regex::new(r"seeds: (.*)").unwrap();
    let seeds: Vec<usize> = re
        .captures(seed_line)
        .unwrap()
        .get(1)
        .map(|m| {
            m.as_str()
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect()
        })
        .unwrap();

    // line offsets in horrible, "I need to use Nom", parser
    let o_seed_to_soil = find_text(&sections, "seed-to-soil map:").unwrap();
    let o_soil_to_fertilizer = find_text(&sections, "soil-to-fertilizer map:").unwrap();
    let o_fertilizer_to_water = find_text(&sections, "fertilizer-to-water map:").unwrap();
    let o_water_to_light = find_text(&sections, "water-to-light map:").unwrap();
    let o_light_to_temperature = find_text(&sections, "light-to-temperature map:").unwrap();
    let o_temperature_to_humidity = find_text(&sections, "temperature-to-humidity map:").unwrap();
    let o_humidity_to_location = find_text(&sections, "humidity-to-location map:").unwrap();

    let seed_to_soil = make_range(sections.clone(), o_seed_to_soil, o_soil_to_fertilizer);
    let soil_to_fertilizer = make_range(
        sections.clone(),
        o_soil_to_fertilizer,
        o_fertilizer_to_water,
    );
    let fertilizer_to_water = make_range(sections.clone(), o_fertilizer_to_water, o_water_to_light);
    let water_to_light = make_range(sections.clone(), o_water_to_light, o_light_to_temperature);
    let light_to_temperature = make_range(
        sections.clone(),
        o_light_to_temperature,
        o_temperature_to_humidity,
    );
    let temperature_to_humidity = make_range(
        sections.clone(),
        o_temperature_to_humidity,
        o_humidity_to_location,
    );
    let humidity_to_location = make_range(
        sections.clone(),
        o_humidity_to_location,
        sections.clone().len() + 1,
    );

    Almanac {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

fn part_two_seeds(seeds: Vec<usize>) -> Vec<Vec<usize>> {
    let groups: Vec<_> = seeds.chunks(2).collect();
    groups
        .iter()
        .map(|group| {
            let start = group.first().unwrap();
            let length = group.get(1).unwrap();
            let range: Vec<usize> = (*start..(*start + length)).collect();
            range
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_parse() {
        let result = parse(&advent_of_code::template::read_file("examples", DAY));

        // this thing is too big to test everything .. observed once, approximated now
        assert_eq!(result.seeds, vec![79, 14, 55, 13]);
        assert_eq!(result.seed_to_soil.first().unwrap().source, 98);
        assert_eq!(result.seed_to_soil.last().unwrap().length, 48);
        assert_eq!(result.humidity_to_location.first().unwrap().source, 56);
        assert_eq!(result.humidity_to_location.last().unwrap().length, 4);
    }

    #[test]
    fn test_lookup_offset() {
        let seed = 79;
        let range: Vec<RangeMap> = vec![RangeMap {
            source: 50,
            destination: 52,
            length: 48,
        }];

        let result = lookup(seed, range);
        assert_eq!(result, 81);
    }

    #[test]
    fn test_lookup_passthrough() {
        let seed = 14;
        let range: Vec<RangeMap> = vec![RangeMap {
            source: 50,
            destination: 52,
            length: 48,
        }];

        let result = lookup(seed, range);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part_two_seeds() {
        let seeds = vec![79, 14, 55, 13];
        let result = part_two_seeds(seeds);

        assert_eq!(result.into_iter().concat().len(), 27);
    }
}
