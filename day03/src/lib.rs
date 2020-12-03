#![cfg_attr(feature = "unstable", feature(test))]

use crate::forest::Forest;

const INPUT: &'static str = include_str!("../input");

mod forest {
    const TREE: u8 = '#' as u8;

    pub struct Forest<'a>(Vec<&'a[u8]>);

    impl <'a> Forest<'a> {
        pub fn parse(input: &str) -> Forest {
            Forest(input.lines().map(str::as_bytes).collect())
        }

        pub fn width(&self) -> usize {
            self.0[0].len()
        }

        pub fn height(&self) -> usize {
            self.0.len()
        }

        pub fn has_tree(&self, x: usize, y: usize) -> bool {
            self.0[y][x] == TREE
        }
    }
}

trait TreeCounter {
    fn count_trees(&self, right: usize, down: usize) -> usize;
}

impl <'a> TreeCounter for Forest<'a> {
    fn count_trees(&self, right: usize, down: usize) -> usize {
        let width = self.width();
        let height = self.height();

        let mut x = right;
        let mut y = down;
        let mut trees = 0;
        while y < height {
            trees += self.has_tree(x, y) as usize;
            y += down;
            x += right;
            x %= width;
        }

        trees
    }
}

fn solve_part_1(forest: &Forest) -> usize {
    forest.count_trees(3, 1)
}

fn solve_part_2(forest: &Forest) -> usize {
    forest.count_trees(1, 1) *
        forest.count_trees(3, 1) *
        forest.count_trees(5, 1) *
        forest.count_trees(7, 1) *
        forest.count_trees(1, 2)
}

pub fn solve() -> (usize, usize) {
    let forest = &Forest::parse(INPUT);
    (solve_part_1(forest), solve_part_2(forest))
}

// ============================================================================================== //

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE_01: &'static str = include_str!("../sample01");

    fn parse_solve_part_1(input: &str) -> usize {
        let forest = &Forest::parse(input);
        solve_part_1(forest)
    }

    fn parse_solve_part_2(input: &str) -> usize {
        let forest = &Forest::parse(input);
        solve_part_2(forest)
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