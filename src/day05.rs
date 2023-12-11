use std::ops::RangeInclusive;
use std::str::Lines;

use itertools::Itertools;

use crate::problem::Problem;

pub struct Day05 {}

impl Problem for Day05 {
    fn part1(&self, lines: Lines) -> String {
        let mut lines = lines;

        let (_, seeds) = lines.next().unwrap().split_once("seeds:").unwrap();
        let seeds = seeds
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect_vec();

        let almanac = Almanac::from_lines_seeds(&mut lines, Seeds::List(seeds));
        almanac.closest_location().to_string()
    }

    fn part2(&self, lines: Lines) -> String {
        let mut lines = lines;
        let (_, seeds) = lines.next().unwrap().split_once("seeds:").unwrap();
        let seeds = Seeds::Ranges(
            seeds
                .split_whitespace()
                .tuples()
                .map(|(s, l)| (s.parse::<u64>().unwrap(), s.parse::<u64>().unwrap()))
                .map(|(s, l)| s..=s + l)
                .collect_vec(),
        );

        let almanac = Almanac::from_lines_seeds(&mut lines, seeds);
        almanac.closest_location().to_string()
    }
}

struct Almanac {
    seeds: Seeds,
    seed_to_soil: RangeMappings,
    soil_to_fertilizer: RangeMappings,
    fertilizer_to_water: RangeMappings,
    water_to_light: RangeMappings,
    light_to_temperature: RangeMappings,
    temperature_to_humidity: RangeMappings,
    humidity_to_location: RangeMappings,
}

struct RangeMappings(Vec<RangeMap>);

struct RangeMap {
    source_range: RangeInclusive<u64>,
    dest_range: RangeInclusive<u64>,
}

impl From<&mut Lines<'_>> for RangeMappings {
    fn from(lines: &mut Lines<'_>) -> Self {
        consume(&mut lines.take_while(|l| l.ends_with(':') || l.trim().is_empty()));
        RangeMappings(
            lines
                .take_while_ref(|l| !l.ends_with(':') && !l.trim().is_empty())
                .map_into()
                .collect_vec(),
        )
    }
}

impl RangeMappings {
    fn map(&self, number: u64) -> u64 {
        for map in self.0.iter() {
            if map.source_range.contains(&number) {
                return map.map(number);
            }
        }

        number
    }
}

impl RangeMap {
    fn map(&self, number: u64) -> u64 {
        if self.source_range.contains(&number) {
            let offset = number - self.source_range.start();
            self.dest_range.start() + offset
        } else {
            number
        }
    }
}

impl From<&str> for RangeMap {
    fn from(str: &str) -> Self {
        let (dest_start, source_start, length) = str
            .split_whitespace()
            .map(str::parse::<u64>)
            .map(Result::unwrap)
            .collect_tuple::<(u64, u64, u64)>()
            .unwrap();

        RangeMap {
            dest_range: dest_start..=(dest_start + length),
            source_range: source_start..=(source_start + length),
        }
    }
}

impl Almanac {
    fn from_lines_seeds(lines: &mut Lines, seeds: Seeds) -> Self {
        let seed_to_soil: RangeMappings = lines.into();
        let soil_to_fertilizer: RangeMappings = lines.into();
        let fertilizer_to_water: RangeMappings = lines.into();
        let water_to_light: RangeMappings = lines.into();
        let light_to_temperature: RangeMappings = lines.into();
        let temperature_to_humidity: RangeMappings = lines.into();
        let humidity_to_location: RangeMappings = lines.into();

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

    fn closest_location(&self) -> u64 {
        let seeds = match &self.seeds {
            Seeds::Ranges(vr) => vr.iter().map(|r| *r.start()).collect_vec(),
            Seeds::List(vr) => vr.clone(),
        };

        seeds
            .iter()
            .map(|seed| {
                self.humidity_to_location.map(
                    self.temperature_to_humidity.map(
                        self.light_to_temperature.map(
                            self.water_to_light
                                .map(self.fertilizer_to_water.map(
                                    self.soil_to_fertilizer.map(self.seed_to_soil.map(*seed)),
                                )),
                        ),
                    ),
                )
            })
            .min()
            .unwrap()
    }
}

enum Seeds {
    Ranges(Vec<RangeInclusive<u64>>),
    List(Vec<u64>),
}

#[inline(always)]
fn consume<I>(iterator: &mut I)
where
    I: Iterator,
{
    for _ in iterator {}
}
