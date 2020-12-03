#![cfg_attr(feature = "unstable", feature(test))]

use day01;
use day02;
use day03;
use std::fmt::Display;

fn solve_day<A, B, S>(day: usize, solver: S) where
    A: Display,
    B: Display,
    S: Fn() -> (A, B)
{
    let (part1, part2) = solver();
    println!("Day {:02}\n    Part 1: {}\n    Part 2: {}", day, part1, part2);
}

fn main() {
    solve_day(01, day01::solve);
    solve_day(02, day02::solve);
    solve_day(03, day03::solve);
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;

    use crate::*;
    use test::Bencher;

    #[bench]
    fn d01(b: &mut Bencher) {
        b.iter(|| {
            day01::solve();
        });
    }

    #[bench]
    fn d02(b: &mut Bencher) {
        b.iter(|| {
            day02::solve();
        });
    }

    #[bench]
    fn d03(b: &mut Bencher) {
        b.iter(|| {
            day03::solve();
        });
    }

    #[bench]
    fn all_days(b: &mut Bencher) {
        b.iter(|| {
            day01::solve();
            day02::solve();
            day03::solve();
        });
    }
}
