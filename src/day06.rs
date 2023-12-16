use std::str::Lines;

use itertools::Itertools;

use crate::problem::Problem;

pub struct Day06 {}

impl Problem for Day06 {
    fn part1(&self, lines: Lines) -> String {
        let text = lines.collect_vec();
        let times = Day06::parse_numbers(text[0]);
        let distance = Day06::parse_numbers(text[1]);
        times
            .zip(distance)
            .map(|(t, d)| Day06::winning_possibilities(t, d))
            .product::<u64>()
            .to_string()
    }

    fn part2(&self, lines: Lines) -> String {
        let text = lines.collect_vec();
        let time = Day06::combine_numbers(text[0]);
        let distance = Day06::combine_numbers(text[1]);
        Day06::winning_possibilities(time, distance).to_string()
    }
}

impl Day06 {
    fn winning_possibilities(race_duration: u64, record_distance: u64) -> u64 {
        let start = record_distance / race_duration;
        let end = (record_distance as f64).sqrt().ceil() as u64;

        for i in start..=end {
            let distance = i * (race_duration - i);
            if distance > record_distance {
                return race_duration - 2 * i + 1;
            }
        }

        unreachable!()
    }

    fn parse_numbers(line: &str) -> impl Iterator<Item = u64> + '_ {
        line.split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(&str::parse::<u64>)
            .map(Result::unwrap)
    }

    fn combine_numbers(line: &str) -> u64 {
        let x = line
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .collect::<String>();

        x.parse::<u64>().unwrap()
    }
}
