#![cfg_attr(feature = "unstable", feature(test))]

const INPUT: &'static str = include_str!("../input");

/// returns Iterator over each group, such that a group is an Iterator over each person
/// a person is a bitmask of each question answered yes
fn parse(input: &str) -> impl Iterator<Item=impl Iterator<Item=usize> + '_> + '_ {
    input.split("\n\n").map(|group| group.lines()
        .map(|person| person.chars()
            .filter(char::is_ascii_lowercase)
            .map(|c| c as usize - 'a' as usize)
            .fold(0usize, |acc, c| acc | (1 << c))))
}

pub fn solve() -> (usize, usize) {
    parse(INPUT)
        .map(|group| group.fold((0, usize::MAX), |(a1, a2), mask| (a1 | mask, a2 & mask)))
        .fold((0, 0), |(a1, a2), (m1, p2)| (a1 + m1.count_ones() as usize, a2 + p2.count_ones() as usize))
}

// ============================================================================================== //

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE_01: &'static str = include_str!("../sample01");
    const SAMPLE_02: &'static str = include_str!("../sample02");

    fn solve_part_1(input: &str) -> usize {
        parse(input)
            .map(|group| group
                .fold(0, |acc, person| acc | person)
                .count_ones() as usize
            )
            .sum()
    }

    fn solve_part_2(input: &str) -> usize {
        parse(input)
            .map(|group| group
                .fold(usize::MAX, |acc, person| acc & person)
                .count_ones() as usize
            )
            .sum()
    }

    #[test]
    fn part_1_sample1_input() {
        let actual = solve_part_1(SAMPLE_01);
        assert_eq!(actual, 6);
    }

    #[test]
    fn part_1_sample2_input() {
        let actual = solve_part_1(SAMPLE_02);
        assert_eq!(actual, 11);
    }

    #[test]
    fn part_1_puzzle_input() {
        let actual = solve_part_1(INPUT);
        assert_eq!(actual, 6778);
    }

    #[test]
    fn part_2_sample2_input() {
        let actual = solve_part_2(SAMPLE_02);
        assert_eq!(actual, 6);
    }

    #[test]
    fn part_2_puzzle_input() {
        let actual = solve_part_2(INPUT);
        assert_eq!(actual, 3406);
    }

    #[test]
    fn solve_puzzle_input() {
        let actual = solve();
        assert_eq!(actual, (6778, 3406));
    }

    #[cfg(all(feature = "unstable", test))]
    mod bench {
        extern crate test;

        use crate::*;
        use crate::tests::*;
        use test::Bencher;

        #[bench]
        fn d06p1(b: &mut Bencher) {
            b.iter(|| {
                solve_part_1(INPUT)
            });
        }

        #[bench]
        fn d06p2(b: &mut Bencher) {
            b.iter(|| {
                solve_part_2(INPUT)
            });
        }

        #[bench]
        fn d06(b: &mut Bencher) {
            b.iter(|| {
                solve();
            });
        }
    }
}