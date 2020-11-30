#![cfg_attr(feature = "unstable", feature(test))]

const INPUT: &'static str = include_str!("../input");

pub fn solve() -> () {}

fn parse(input: &str) -> () {}

fn solution() -> () {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use test::Bencher;
    use crate::{solution, parse, INPUT};

    #[bench]
    fn day01_parse(b: &mut Bencher) {
        b.iter(|| {
            parse(&INPUT);
        });
    }

    #[bench]
    fn day01_solve(b: &mut Bencher) {
        b.iter(|| {
            solution();
        });
    }
}