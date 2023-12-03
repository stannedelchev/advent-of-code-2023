extern crate core;

use crate::day01::Day01;

mod day01;

fn main() {
    let input = std::fs::read_to_string("inputs/input01.txt").unwrap();

    let day01 = Day01 {};
    let part1 = day01.part1(input.lines());
    println!("{:?}", part1);
    let part2 = day01.part2(input.lines());
    println!("{:?}", part2);
}
