#![cfg_attr(feature = "unstable", feature(test))]

const INPUT: &'static str = include_str!("../input");
const TREE: u8 = '#' as u8;

fn count_trees(input: &str, right: usize, down: usize) -> usize {
    let lines: Box<[&[u8]]> = input.lines().map(str::as_bytes).collect();
    let width = lines[0].len();
    let height = lines.len();

    let mut x = right;
    let mut y = down;
    let mut trees = 0;
    while y < height {
        trees += (lines[y][x] == TREE) as usize;
        y += down;
        x += right;
        x %= width;
    }

    trees
}

fn solve_part_1(input: &str) -> usize {
    count_trees(input, 3, 1)
}

fn solve_part_2(input: &str) -> usize {
    count_trees(input, 1, 1) *
        count_trees(input, 3, 1) *
        count_trees(input, 5, 1) *
        count_trees(input, 7, 1) *
        count_trees(input, 1, 2)
}

pub fn solve() -> (usize, usize) {
    (solve_part_1(INPUT), solve_part_2(INPUT))
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
        fn d03p1(b: &mut Bencher) {
            b.iter(|| {
                solve_part_1(INPUT);
            });
        }

        #[bench]
        fn d03p2(b: &mut Bencher) {
            b.iter(|| {
                solve_part_2(INPUT);
            });
        }
    }
}