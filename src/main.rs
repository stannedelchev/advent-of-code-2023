extern crate core;

use std::env;
use std::env::Args;
use std::ops::Div;
use std::time::{Duration, Instant};

use crate::day01::Day01;
use crate::day02::Day02;
use crate::day03::Day03;
use crate::day04::Day04;
use crate::day05::Day05;
use crate::problem::Problem;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod problem;

fn main() {
    let part_runs = parse_run_args(env::args());
    let problems: Vec<&dyn Problem> = vec![&Day01 {}, &Day02 {}, &Day03 {}, &Day04 {}, &Day05 {}];

    println!("Running each part {} time(s).", part_runs);

    for (day, problem) in problems.iter().enumerate() {
        let day = day + 1; // iterator indexes start from 0, but days start from 1
        let input = std::fs::read_to_string(format!("inputs/input{:02}.txt", day)).unwrap();
        let fmt = |t: &TimedRunSet| {
            format!(
                "Day {:02} Part 1: {} in {:.4}ms, min: {:.4}ms, max: {:.4}ms",
                day,
                t.get_result(),
                t.avg.as_secs_f64() * 1000.0,
                t.min.as_secs_f64() * 1000.0,
                t.max.as_secs_f64() * 1000.0
            )
        };

        run_part(part_runs, || problem.part1(input.lines()), |t| fmt(t));
        run_part(part_runs, || problem.part2(input.lines()), |t| fmt(t));
    }
}

fn parse_run_args(args: Args) -> u32 {
    let args = args.collect::<Vec<String>>();
    args.get(1)
        .unwrap_or(&"1".to_string())
        .parse::<u32>()
        .unwrap_or(1)
}

fn run_part<F, FFmt>(runs: u32, f: F, fmt: FFmt)
where
    F: Fn() -> String,
    FFmt: Fn(&TimedRunSet) -> String,
{
    let runs = TimedRunSet::new(
        (0..runs)
            .map(|_| TimedRun::run(|| f()))
            .collect::<Vec<TimedRun>>(),
    );
    println!("{}", fmt(&runs));
}

struct TimedRun {
    result: String,
    time: Duration,
}

impl TimedRun {
    pub fn run<F>(f: F) -> TimedRun
    where
        F: Fn() -> String,
    {
        let start = Instant::now();
        let result = f();
        let time = start.elapsed();

        TimedRun { result, time }
    }
}

struct TimedRunSet {
    runs: Vec<TimedRun>,
    min: Duration,
    avg: Duration,
    max: Duration,
}

impl TimedRunSet {
    pub fn new(runs: Vec<TimedRun>) -> TimedRunSet {
        let min = runs.iter().map(|r| r.time).min().unwrap();
        let max = runs.iter().map(|r| r.time).max().unwrap();
        let avg = runs
            .iter()
            .map(|r| r.time)
            .sum::<Duration>()
            .div(runs.len() as u32);
        TimedRunSet {
            runs,
            min,
            avg,
            max,
        }
    }

    pub fn get_result<'a>(self: &'a Self) -> &'a String {
        let first_run = self.runs.get(0).unwrap();
        &first_run.result
    }
}
