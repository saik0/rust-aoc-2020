#![cfg_attr(feature = "unstable", feature(test))]

const INPUT: &'static str = include_str!("../input");
const TREE: u8 = '#' as u8;

fn parse_lines(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

fn count_trees(lines: &[&[u8]], right: usize, down: usize) -> usize {
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

fn solve_part_1(lines: &[&[u8]]) -> usize {
    count_trees(lines, 3, 1)
}

fn solve_part_2(lines: &[&[u8]]) -> usize {
    count_trees(lines, 1, 1) *
        count_trees(lines, 3, 1) *
        count_trees(lines, 5, 1) *
        count_trees(lines, 7, 1) *
        count_trees(lines, 1, 2)
}

pub fn solve() -> (usize, usize) {
    let lines = &parse_lines(INPUT);
    (solve_part_1(lines), solve_part_2(lines))
}

// ============================================================================================== //

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE_01: &'static str = include_str!("../sample01");

    fn parse_solve_part_1(input: &str) -> usize {
        let lines = &parse_lines(input);
        solve_part_1(lines)
    }

    fn parse_solve_part_2(input: &str) -> usize {
        let lines = &parse_lines(input);
        solve_part_2(lines)
    }

    #[test]
    fn part_1_sample_input() {
        let count = parse_solve_part_1(SAMPLE_01);
        assert_eq!(count, 7);
    }

    #[test]
    fn part_1_puzzle_input() {
        let count = parse_solve_part_1(INPUT);
        assert_eq!(count, 191);
    }

    #[test]
    fn part_2_sample_input() {
        let count = parse_solve_part_2(SAMPLE_01);
        assert_eq!(count, 336);
    }

    #[test]
    fn part_2_puzzle_input() {
        let count = parse_solve_part_2(INPUT);
        assert_eq!(count, 1478615040);
    }

    #[cfg(all(feature = "unstable", test))]
    mod bench {
        extern crate test;

        use crate::*;
        use crate::tests::*;
        use test::Bencher;

        #[bench]
        fn d03p1(b: &mut Bencher) {
            b.iter(|| {
                parse_solve_part_1(INPUT);
            });
        }

        #[bench]
        fn d03p2(b: &mut Bencher) {
            b.iter(|| {
                parse_solve_part_2(INPUT);
            });
        }
    }
}