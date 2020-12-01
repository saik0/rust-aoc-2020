#![cfg_attr(feature = "unstable", feature(test))]

const TARGET_YEAR: usize = 2020;
const INPUT: &'static str = include_str!("../input");

trait Parser {
    fn parse(input: &str) -> Vec<usize>;
}

pub struct UnsortedParser;

impl Parser for UnsortedParser {
    fn parse(input: &str) -> Vec<usize> {
        input.lines()
            .map(|line| {
                line.parse::<usize>().expect("failed to parse line")
            })
            .collect()
    }
}

pub struct SortedParser;

impl Parser for SortedParser {
    fn parse(input: &str) -> Vec<usize> {
        let mut vec = UnsortedParser::parse(input);
        vec.sort_unstable();
        vec
    }
}

// ============================================================================================== //

trait SearchAlg {
    type Parser: Parser;

    fn search(&self, expenses: &[usize], sum: usize) -> Option<usize>;
}

struct BruteForce;

impl SearchAlg for BruteForce {
    type Parser = UnsortedParser;

    /// Search [expenses] for two values that sum to [sum] using a brute force algorithm
    /// returns the product of the values
    fn search(&self, expenses: &[usize], sum: usize) -> Option<usize> {
        for i in 0..(expenses.len() - 1) {
            for j in (i + 1)..expenses.len() {
                if expenses[i] + expenses[j] == sum {
                    return Some(expenses[i] * expenses[j]);
                }
            }
        }

        None
    }
}

struct TwoPointer;

impl SearchAlg for TwoPointer {
    type Parser = SortedParser;

    /// Search _sorted_ [expenses] for two values that sum to [sum] using a two pointer algorithm
    /// returns the product of the values
    fn search(&self, expenses: &[usize], sum: usize) -> Option<usize> {
        let mut l = 0;
        let mut r = expenses.len() - 1;

        while l < r {
            if expenses[l] + expenses[r] == sum {
                return Some(expenses[l] * expenses[r]);
            } else if expenses[l] + expenses[r] < sum {
                l += 1;
            } else {
                r -= 1;
            }
        }

        None
    }
}

// ============================================================================================== //

trait Part {
    fn solve(&self, expenses: &[usize], search_alg: &impl SearchAlg) -> Option<usize>;
}

struct Part1;

impl Part for Part1 {
    fn solve(&self, expenses: &[usize], search_alg: &impl SearchAlg) -> Option<usize> {
        search_alg.search(expenses, TARGET_YEAR)
    }
}

struct Part2;

impl Part for Part2 {
    fn solve(&self, expenses: &[usize], search_alg: &impl SearchAlg) -> Option<usize> {
        expenses.iter().take(expenses.len() - 2).enumerate().find_map(|(i, &a)|
            search_alg.search(&expenses[i + 1..], TARGET_YEAR - a).map(|b| a * b)
        )
    }
}

// ============================================================================================== //

// Solve both parts

fn parse_and_solve_both_parts<P: Parser>(
    input: &str,
    search_alg: &impl SearchAlg<Parser=P>,
) -> (usize, usize) {
    let expenses = P::parse(input);
    (
        Part1.solve(&expenses, search_alg).unwrap(),
        Part2.solve(&expenses, search_alg).unwrap(),
    )
}

pub fn solve() -> (usize, usize) {
    parse_and_solve_both_parts(&INPUT, &TwoPointer)
}

// ============================================================================================== //

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE_01: &'static str = include_str!("../sample01");

    /// Helper for tests / benches
    fn parse_and_solve_part<P: Parser>(
        input: &str,
        solver: impl Part,
        search_alg: impl SearchAlg<Parser=P>,
    ) -> usize {
        solver.solve(&P::parse(input), &search_alg).unwrap()
    }

    #[test]
    fn part_01_brute_force_sample_01() {
        let actual = parse_and_solve_part(SAMPLE_01, Part1, BruteForce);
        assert_eq!(actual, 514579);
    }

    #[test]
    fn part_01_brute_force_puzzle_input() {
        let actual = parse_and_solve_part(INPUT, Part1, BruteForce);
        assert_eq!(actual, 224436);
    }


    #[test]
    fn part01_two_pointer_sample_01() {
        let actual = parse_and_solve_part(SAMPLE_01, Part1, TwoPointer);
        assert_eq!(actual, 514579);
    }

    #[test]
    fn part01_two_pointer_puzzle_input() {
        let actual = parse_and_solve_part(INPUT, Part1, TwoPointer);
        assert_eq!(actual, 224436);
    }

    #[test]
    fn part_2_brute_force_sample_01() {
        let actual = parse_and_solve_part(SAMPLE_01, Part2, BruteForce);
        assert_eq!(actual, 241861950);
    }

    #[test]
    fn part_2_brute_force_puzzle_input() {
        let actual = parse_and_solve_part(INPUT, Part2, BruteForce);
        assert_eq!(actual, 303394260);
    }

    #[test]
    fn part_2_two_pointer_sample_01() {
        let actual = parse_and_solve_part(SAMPLE_01, Part2, TwoPointer);
        assert_eq!(actual, 241861950);
    }

    #[test]
    fn part_2_two_pointer_puzzle_input() {
        let actual = parse_and_solve_part(INPUT, Part2, TwoPointer);
        assert_eq!(actual, 303394260);
    }

    #[cfg(all(feature = "unstable", test))]
    mod bench {
        extern crate test;

        use crate::*;
        use tests::parse_and_solve_part;
        use test::Bencher;

        #[bench]
        fn d01p1_brute_force(b: &mut Bencher) {
            b.iter(|| {
                parse_and_solve_part(INPUT, Part1, BruteForce)
            });
        }

        #[bench]
        fn d01p1_two_pointer(b: &mut Bencher) {
            b.iter(|| {
                parse_and_solve_part(INPUT, Part1, TwoPointer)
            });
        }

        #[bench]
        fn d01p2_brute_force(b: &mut Bencher) {
            b.iter(|| {
                parse_and_solve_part(INPUT, Part2, BruteForce)
            });
        }

        #[bench]
        fn d01p2_two_pointer(b: &mut Bencher) {
            b.iter(|| {
                parse_and_solve_part(INPUT, Part2, TwoPointer)
            });
        }

        #[bench]
        fn d01p1p2_brute_force(b: &mut Bencher) {
            b.iter(|| {
                parse_and_solve_both_parts(INPUT, &BruteForce)
            });
        }

        #[bench]
        fn d01p1p2_two_pointer(b: &mut Bencher) {
            b.iter(|| {
                parse_and_solve_both_parts(INPUT, &TwoPointer)
            });
        }
    }
}