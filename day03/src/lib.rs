#![cfg_attr(feature = "unstable", feature(test))]

const INPUT: &'static str = include_str!("../input");

pub fn solve() -> (usize, usize) {
    (solve_part_1(INPUT), solve_part_2(INPUT))
}

fn solution(input: &str, r: usize, d: usize) -> usize {
    let w = input.lines().nth(0).unwrap().len();
    let h = input.lines().count();

    let mut x = r;
    let mut y = d;
    let mut trees = 0;
    while y < h {
        let c = input.lines().nth(y).unwrap().chars().nth(x).unwrap();
        trees += (c == '#') as usize;
        x += r;
        x %= w;
        y += d;
    }

    trees
}

fn solve_part_1(input: &str) -> usize {
    solution(input, 3, 1)
}

fn solve_part_2(input: &str) -> usize {
    solution(input, 1, 1) *
        solution(input, 3, 1) *
        solution(input, 5, 1) *
        solution(input, 7, 1) *
        solution(input, 1, 2)
}

// ============================================================================================== //

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE_01: &'static str = include_str!("../sample01");


    #[test]
    fn part_1_sample_input() {
        let count = solve_part_1(SAMPLE_01);
        assert_eq!(count, 7);
    }

    #[test]
    fn part_1_puzzle_input() {
        let count = solve_part_1(INPUT);
        assert_eq!(count, 191);
    }

    #[test]
    fn part_2_sample_input() {
        let count = solve_part_2(SAMPLE_01);
        assert_eq!(count, 336);
    }

    #[test]
    fn part_2_puzzle_input() {
        let count = solve_part_2(INPUT);
        assert_eq!(count, 1478615040);
    }

    #[cfg(all(feature = "unstable", test))]
    mod bench {
        extern crate test;

        use crate::*;
        use test::Bencher;

        #[bench]
        fn d03p1_brute_force(b: &mut Bencher) {
            b.iter(|| {});
        }
    }
}