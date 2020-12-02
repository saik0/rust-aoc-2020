#![cfg_attr(feature = "unstable", feature(test))]

use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &'static str = include_str!("../input");
const PASSWORD_PATTERN: &'static str = r"(\d+)-(\d+) ([a-z]): ([a-z]+)";

lazy_static! {
    static ref PASSWORD_REGEX: Regex = Regex::new(PASSWORD_PATTERN).unwrap();
}

#[derive(Debug)]
struct PasswordDbEntry {
    low: usize,
    high: usize,
    char: char,
    password: String,
}

impl PasswordDbEntry {
    fn parse(line: &str) -> PasswordDbEntry {
        let captures = PASSWORD_REGEX.captures(line).expect("invalid password line");

        PasswordDbEntry {
            low: captures[1].parse().unwrap(),
            high: captures[2].parse().unwrap(),
            char: captures[3].parse().unwrap(),
            password: captures[4].to_owned(),
        }
    }

    fn is_part_1_valid(&self) -> bool {
        let char_count = self.password.chars().filter(|&it| it == self.char).count();
        (self.low..=self.high).contains(&char_count)
    }

    fn is_part_2_valid(&self) -> bool {
        let mut chars = self.password.chars();
        let lo = chars.nth(self.low - 1).expect("low out of range");
        let hi = chars.nth(self.high - self.low - 1).expect("high out of range");
        (lo == self.char) ^ (hi == self.char)
    }
}

pub fn solve() -> (usize, usize) {
    INPUT.lines()
        .map(PasswordDbEntry::parse)
        .fold((0, 0), |(p1, p2), entry|
            (p1 + entry.is_part_1_valid() as usize, p2 + entry.is_part_2_valid() as usize)
        )
}

fn solve_part_1(input: &str) -> usize {
    input.lines()
        .map(PasswordDbEntry::parse)
        .filter(PasswordDbEntry::is_part_1_valid)
        .count()
}

fn solve_part_2(input: &str) -> usize {
    input.lines()
        .map(PasswordDbEntry::parse)
        .filter(PasswordDbEntry::is_part_2_valid)
        .count()
}

// ============================================================================================== //

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE_01: &'static str = include_str!("../sample01");

    #[test]
    fn parse_entry() {
        let first = SAMPLE_01.lines().next().unwrap();
        let entry = PasswordDbEntry::parse(first);

        assert_eq!(entry.low, 1);
        assert_eq!(entry.high, 3);
        assert_eq!(entry.char, 'a');
        assert_eq!(entry.password, "abcde");
    }

    #[test]
    fn part_1_sample_input() {
        let entries: Vec<PasswordDbEntry> = SAMPLE_01.lines().map(PasswordDbEntry::parse).collect();

        assert_eq!(entries[0].is_part_1_valid(), true);
        assert_eq!(entries[1].is_part_1_valid(), false);
        assert_eq!(entries[2].is_part_1_valid(), true);
    }

    #[test]
    fn part_1_puzzle_input() {
        assert_eq!(solve_part_1(INPUT), 493);
    }

    #[test]
    fn part_2_sample_input() {
        let entries: Vec<PasswordDbEntry> = SAMPLE_01.lines().map(PasswordDbEntry::parse).collect();

        println!("{:#?}", entries);

        assert_eq!(entries[0].is_part_2_valid(), true);
        assert_eq!(entries[1].is_part_2_valid(), false);
        assert_eq!(entries[2].is_part_2_valid(), false);
    }

    #[test]
    fn part_2_puzzle_input() {
        assert_eq!(solve_part_2(INPUT), 593);
    }

    #[cfg(all(feature = "unstable", test))]
    mod bench {
        extern crate test;

        use crate::*;
        use test::Bencher;

        #[bench]
        fn d02p1_solve(b: &mut Bencher) {
            b.iter(|| {
                solve_part_1(INPUT);
            });
        }

        #[bench]
        fn d02p2_solve(b: &mut Bencher) {
            b.iter(|| {
                solve_part_2(INPUT);
            });
        }
    }
}