#![cfg_attr(feature = "unstable", feature(test))]

const INPUT: &'static str = include_str!("../input");

pub fn solve() -> (&'static str, &'static str) {
    ("", "")
}

// ============================================================================================== //

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE_01: &'static str = include_str!("../sample01");

    #[test]
    fn part_1_sample_input() {
        assert_eq!(0, 1);
    }

    #[test]
    fn part_1_puzzle_input() {
        assert_eq!(0, 1);
    }

    #[test]
    fn part_2_sample_input() {
        let actual = solve_part_2(SAMPLE_01);
        assert_eq!(actual, 2);
    }

    #[test]
    fn part_2_puzzle_input() {
        let actual = solve_part_2(INPUT);
        assert_eq!(actual, 172);
    }

    #[cfg(all(feature = "unstable", test))]
    mod bench {
        extern crate test;

        use crate::*;
        use test::Bencher;

        #[bench]
        fn d03p1_brute_force(b: &mut Bencher) {
            b.iter(|| {

            });
        }
    }
}