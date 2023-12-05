use std::str::Lines;

pub trait Problem {
    fn part1(self: &Self, lines: Lines) -> String;
    fn part2(self: &Self, lines: Lines) -> String;
}
