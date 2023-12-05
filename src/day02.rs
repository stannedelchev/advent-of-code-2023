use std::str::Lines;

use crate::problem::Problem;

pub struct Day02 {}

impl Problem for Day02 {
    fn part1(&self, lines: Lines) -> String {
        lines
            .map(Day02::get_id_and_subsets)
            .filter(|m| {
                let (_, cube_subsets) = m;
                let max_red = Day02::max_for_color(cube_subsets, "red");
                let max_green = Day02::max_for_color(cube_subsets, "green");
                let max_blue = Day02::max_for_color(cube_subsets, "blue");
                *max_red <= 12 && *max_green <= 13 && *max_blue <= 14
            })
            .map(|(game_id, _)| game_id)
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, lines: Lines) -> String {
        lines
            .map(Day02::get_id_and_subsets)
            .map(|m| {
                let (_, cube_subsets) = m;
                let max_red = Day02::max_for_color(&cube_subsets, "red");
                let max_green = Day02::max_for_color(&cube_subsets, "green");
                let max_blue = Day02::max_for_color(&cube_subsets, "blue");
                max_red * max_green * max_blue
            })
            .sum::<u32>()
            .to_string()
    }
}

impl Day02 {
    fn get_id_and_subsets(line: &str) -> (u32, Vec<(u32, &str)>) {
        let parts = line
            .split(&[':', ';', ','])
            .map(str::trim)
            .collect::<Vec<&str>>();

        let game = parts[0];
        let game_id = game.split(' ').nth(1).unwrap().parse::<u32>().unwrap();

        let cube_subsets = parts[1..]
            .iter()
            .map(|s| {
                let split = s.split(' ').collect::<Vec<&str>>();
                (split[0].parse::<u32>().unwrap(), split[1])
            })
            .collect::<Vec<(u32, &str)>>();

        (game_id, cube_subsets)
    }

    fn max_for_color<'a>(cube_subsets: &'a Vec<(u32, &str)>, color: &str) -> &'a u32 {
        cube_subsets
            .iter()
            .filter(|(_, c)| *c == color)
            .map(|(i, _)| i)
            .max()
            .unwrap()
    }
}
