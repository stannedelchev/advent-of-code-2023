use std::str::{FromStr, Lines};

use itertools::Itertools;
use tinyset::Set64;

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
        let cards = lines.map(|l| l.parse::<Card>().unwrap()).collect_vec();
        let mut cards_count = vec![1; cards.len()];
        let mut total = 0;

        for (idx, card) in cards.iter().enumerate() {
            let matches = card.matches();
            for i in 1..=matches {
                cards_count[idx + i as usize] += cards_count[idx];
            }

            total += cards_count[idx];
        }

        total.to_string()
    }
}

impl Day04 {
    fn numbers(str: &str) -> Set64<u32> {
        Set64::from_iter(
            str.split_whitespace()
                .map(str::parse::<u32>)
                .map(Result::unwrap),
        )
    }
}

struct Card {
    winning_numbers: Set64<u32>,
    my_numbers: Set64<u32>,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let winning_numbers = Day04::numbers(&s[9..40]);
        let my_numbers = Day04::numbers(&s[41..]);

        Ok(Card {
            winning_numbers,
            my_numbers,
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

    fn matches(&self) -> u32 {
        self.winning_numbers
            .iter()
            .filter(|n| self.my_numbers.contains(n))
            .count() as u32
    }
}
