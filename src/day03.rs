use std::iter::Peekable;
use std::str::{CharIndices, Lines};

use itertools::Itertools;

use crate::problem::Problem;

pub struct Day03 {}

impl Problem for Day03 {
    fn part1(&self, lines: Lines) -> String {
        let parsed = lines
            .enumerate()
            .map(|(idx, line)| line.engine_parts(idx).collect::<Vec<EnginePartOrSymbol>>())
            .flatten();

        let (symbols, parts) = Day03::symbols_and_parts(parsed);

        symbols
            .iter()
            .map(|s| {
                parts
                    .iter()
                    .filter(|p| p.adjacent(s))
                    .map(|p| p.data)
                    .sum::<usize>()
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, lines: Lines) -> String {
        let parsed = lines
            .enumerate()
            .map(|(idx, line)| line.engine_parts(idx).collect::<Vec<EnginePartOrSymbol>>())
            .flatten();

        let (symbols, parts) = Day03::symbols_and_parts(parsed);

        symbols
            .iter()
            .filter(|s| s.data == '*')
            .filter_map(|s| {
                let neighbours = parts
                    .iter()
                    .filter(|p| p.adjacent(s))
                    .collect::<Vec<&EnginePart>>();
                match neighbours.len() {
                    2 => Some(neighbours[0].data * neighbours[1].data),
                    _ => None,
                }
            })
            .sum::<usize>()
            .to_string()
    }
}

impl Day03 {
    fn symbols_and_parts<I>(data: I) -> (Vec<Symbol>, Vec<EnginePart>)
    where
        I: Iterator<Item = EnginePartOrSymbol>,
    {
        let mut parts: Vec<EnginePart> = vec![];
        let mut symbols: Vec<Symbol> = vec![];

        for part_or_symbol in data {
            match part_or_symbol {
                EnginePartOrSymbol::Part(p) => parts.push(p),
                EnginePartOrSymbol::Symbol(s) => symbols.push(s),
            }
        }

        (symbols, parts)
    }
}

enum EnginePartOrSymbol {
    Part(EnginePart),
    Symbol(Symbol),
}

trait EnginePartParser {
    fn engine_parts(self: &Self, y: usize) -> EnginePartIterator;
}

impl EnginePartParser for &str {
    fn engine_parts<'a>(self: &'a Self, y: usize) -> EnginePartIterator<'a> {
        EnginePartIterator::new(self, y)
    }
}

struct EnginePartIterator<'a> {
    y: usize,
    line_chars: Peekable<CharIndices<'a>>,
}

impl<'a> EnginePartIterator<'a> {
    fn new(line: &str, y: usize) -> EnginePartIterator {
        EnginePartIterator {
            y,
            line_chars: line.char_indices().peekable(),
        }
    }
}

impl Iterator for EnginePartIterator<'_> {
    type Item = EnginePartOrSymbol;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&(idx, c)) = self.line_chars.peek() {
            match c {
                '.' => {
                    self.line_chars.next();
                    continue;
                }
                '0'..='9' => {
                    let data = self
                        .line_chars
                        .take_while_ref(|(_, ch)| ch.is_numeric())
                        .map(|(_, ch)| ch)
                        .collect::<String>();
                    let end_x = idx + data.len();
                    let data = data.parse().unwrap();
                    return Some(EnginePartOrSymbol::Part(EnginePart {
                        data,
                        start_x: idx,
                        end_x,
                        y: self.y,
                    }));
                }
                c => {
                    self.line_chars.next();
                    return Some(EnginePartOrSymbol::Symbol(Symbol {
                        data: c,
                        x: idx,
                        y: self.y,
                    }));
                }
            }
        }

        None
    }
}

#[derive(Debug)]
struct EnginePart {
    data: usize,
    start_x: usize,
    end_x: usize,
    y: usize,
}

#[derive(Debug)]
struct Symbol {
    data: char,
    x: usize,
    y: usize,
}

impl EnginePart {
    fn adjacent(&self, s: &Symbol) -> bool {
        // part and symbol on same row
        if s.y == self.y {
            return s.x == self.end_x || (s.x as isize == self.start_x as isize - 1);
        }

        if s.y as isize == self.y as isize - 1 || s.y == self.y + 1 {
            return (self.start_x as isize - 1..self.end_x as isize + 1).contains(&(s.x as isize));
        }

        false
    }
}
