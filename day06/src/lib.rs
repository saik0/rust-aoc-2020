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
        assert_eq!(0, 1);
    }

    #[test]
    fn part_2_puzzle_input() {
        assert_eq!(0, 1);
    }

    #[cfg(all(feature = "unstable", test))]
    mod bench {
        extern crate test;

        use crate::*;
        use test::Bencher;

        #[bench]
        fn d06(b: &mut Bencher) {
            b.iter(|| {

            });
        }
    }
}