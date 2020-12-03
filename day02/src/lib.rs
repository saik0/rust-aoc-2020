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
    fn parse(line: &'a str) -> Option<PasswordDbEntry<'a>> {
        let mut split = line.split(&[' ', '-', ':'][..]);

        Some(
            PasswordDbEntry {
                low: split.next()?.parse().ok()?,
                high: split.next()?.parse().ok()?,
                char: split.next()?.parse().ok()?,
                password: split.skip(1).next()?,
            }
        )
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
        .filter_map(PasswordDbEntry::parse)
        .fold((0, 0), |(p1, p2), entry|
            (p1 + entry.is_part_1_valid() as usize, p2 + entry.is_part_2_valid() as usize)
        )
}

fn solve_part_1(input: &str) -> usize {
    input.lines()
        .filter_map(PasswordDbEntry::parse)
        .filter(PasswordDbEntry::is_part_1_valid)
        .count()
}

fn solve_part_2(input: &str) -> usize {
    input.lines()
        .filter_map(PasswordDbEntry::parse)
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
        let entry = PasswordDbEntry::parse(first).unwrap();

        assert_eq!(entry.low, 1);
        assert_eq!(entry.high, 3);
        assert_eq!(entry.char, 'a');
        assert_eq!(entry.password, "abcde");
    }

    #[test]
    fn part_1_sample_input() {
        let mut entries = SAMPLE_01.lines().filter_map(PasswordDbEntry::parse);

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
        let mut entries = SAMPLE_01.lines().filter_map(PasswordDbEntry::parse);

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