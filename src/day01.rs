use std::str::Lines;

pub struct Day01 {}

impl Day01 {
    pub fn part1(&self, lines: Lines) -> String {
        lines
            .map(Day01::line_to_calculation_value)
            .sum::<u32>()
            .to_string()
    }

    pub fn part2(&self, lines: Lines) -> String {
        "".to_string()
    }

    fn line_to_calculation_value(line: &str) -> u32 {
        let mut numbers = line.chars().filter(|c| c.is_numeric());
        let first_digit = numbers.next().map(Day01::parse_char).unwrap_or(0);
        let last_digit = numbers
            .last()
            .map(Day01::parse_char)
            .unwrap_or(first_digit.clone());
        first_digit * 10 + last_digit
    }

    fn parse_char(c: char) -> u32 {
        c.to_digit(10).unwrap()
    }
}
