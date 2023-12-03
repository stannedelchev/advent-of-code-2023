use std::str::Lines;

use aho_corasick::automaton::Candidate::Match;
use aho_corasick::AhoCorasick;

pub struct Day01 {}

impl Day01 {
    pub fn part1(&self, lines: Lines) -> String {
        lines
            .map(|line| {
                let mut numbers = line.chars().filter(|c| c.is_numeric());
                let first_digit = numbers.next().map(Day01::parse_char).unwrap_or(0);
                let last_digit = numbers
                    .last()
                    .map(Day01::parse_char)
                    .unwrap_or(first_digit.clone());
                first_digit * 10 + last_digit
            })
            .sum::<u32>()
            .to_string()
    }

    fn parse_char(c: char) -> u32 {
        c.to_digit(10).unwrap()
    }

    pub fn part2(&self, lines: Lines) -> String {
        let patterns = &[
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "one", "two", "three", "four",
            "five", "six", "seven", "eight", "nine",
        ];
        let regex = AhoCorasick::new(patterns).unwrap();
        lines
            .map(|line| {
                let matches = regex
                    .find_overlapping_iter(line)
                    .map(|m| &line[m.start()..m.end()])
                    .collect::<Vec<&str>>();
                let first_digit = Day01::parse_digit(matches[0]);
                let last_digit = Day01::parse_digit(matches[matches.len() - 1]);
                first_digit * 10 + last_digit
            })
            .sum::<u32>()
            .to_string()
    }

    fn parse_digit(str: &str) -> u32 {
        match str {
            "1" | "one" => 1,
            "2" | "two" => 2,
            "3" | "three" => 3,
            "4" | "four" => 4,
            "5" | "five" => 5,
            "6" | "six" => 6,
            "7" | "seven" => 7,
            "8" | "eight" => 8,
            "9" | "nine" => 9,
            _ => 0,
        }
    }
}
