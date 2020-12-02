#![cfg_attr(feature = "unstable", feature(test))]

const INPUT: &'static str = include_str!("../input");

#[derive(Debug)]
struct PasswordDbEntry<'a> {
    low: usize,
    high: usize,
    char: char,
    password: &'a str,
}

impl <'a> PasswordDbEntry<'a> {
    fn parse(line: &'a str) -> PasswordDbEntry<'a> {
        let low_end = line.find('-').unwrap();
        let high_start = low_end+1;
        let high_end = high_start + line[high_start..].find(' ').unwrap();

        PasswordDbEntry {
            low: line[..low_end].parse().unwrap(),
            high: line[high_start..high_end].parse().unwrap(),
            char: line[high_end+1..high_end+2].parse().unwrap(),
            password: &line[high_end+4..],
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
        let mut entries = SAMPLE_01.lines().map(PasswordDbEntry::parse);

        assert_eq!(entries.next().unwrap().is_part_1_valid(), true);
        assert_eq!(entries.next().unwrap().is_part_1_valid(), false);
        assert_eq!(entries.next().unwrap().is_part_1_valid(), true);
    }

    #[test]
    fn part_1_puzzle_input() {
        assert_eq!(solve_part_1(INPUT), 493);
    }

    #[test]
    fn part_2_sample_input() {
        let mut entries = SAMPLE_01.lines().map(PasswordDbEntry::parse);

        assert_eq!(entries.next().unwrap().is_part_2_valid(), true);
        assert_eq!(entries.next().unwrap().is_part_2_valid(), false);
        assert_eq!(entries.next().unwrap().is_part_2_valid(), false);
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