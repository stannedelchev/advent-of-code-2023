use std::collections::HashSet;
use std::str::{FromStr, Lines};

use crate::problem::Problem;

pub struct Day04 {}

impl Problem for Day04 {
    fn part1(&self, lines: Lines) -> String {
        lines
            .map(str::parse::<Card>)
            .map(Result::unwrap)
            .map(|c| c.points())
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, lines: Lines) -> String {
        "".to_string()
    }
}

impl Day04 {
    fn numbers(str: &str) -> HashSet<u32> {
        HashSet::from_iter(
            str.split_whitespace()
                .map(str::parse::<u32>)
                .map(Result::unwrap),
        )
    }
}

struct Card {
    number: u32,
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
    points: Option<u32>,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number = s[4..8].trim().parse::<u32>().unwrap();
        let winning_numbers = Day04::numbers(&s[9..40]);
        let my_numbers = Day04::numbers(&s[41..]);
        Ok(Card {
            number,
            winning_numbers,
            my_numbers,
            points: None,
        })
    }
}

impl Card {
    fn points(&self) -> u32 {
        self.winning_numbers
            .iter()
            .filter(|n| self.my_numbers.contains(n))
            .fold(0_u32, |acc, _| if acc == 0 { 1 } else { acc * 2 })
    }

    fn points_mut(&mut self) {
        self.points = Some(self.points());
    }
}
