use day01;
use day02;
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
}
