#![cfg_attr(feature = "unstable", feature(test))]

pub fn solve() -> ((), ()) {
    ((), ())
}

// ============================================================================================== //

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE_01: &'static str = include_str!("../sample01");


    #[test]
    fn part_2_sample_input() {
        assert_eq!(0, 0);
    }

    #[cfg(all(feature = "unstable", test))]
    mod bench {
        extern crate test;

        use crate::*;
        use test::Bencher;

        #[bench]
        fn d02p1_brute_force(b: &mut Bencher) {
            b.iter(|| {

            });
        }
    }
}