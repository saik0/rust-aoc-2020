#![cfg_attr(feature = "unstable", feature(test))]

const INPUT: &'static str = include_str!("../input");

fn decode_binary_space(input: &str, lo: char, hi: char, r: usize) -> usize {
    let mut r = r;
    let mut l = 0;

    for c in input.chars() {
        let m = (l + (r - 1)) / 2;
        if c == lo { r = m } else if c == hi { l = m + 1 };
    }

    l
}

fn calc_seat_id(row: usize, col: usize) -> usize {
    row * 8 + col
}

fn decode(input: &str) -> usize {
    let (enc_row, enc_col) = input.split_at(7);
    let row = decode_binary_space(enc_row, 'F', 'B', 127);
    let col = decode_binary_space(enc_col, 'L', 'R', 7);
    let seat_id = calc_seat_id(row, col);
    seat_id
}

pub fn solve() -> (usize, usize) {
    let mut seat_ids = INPUT.lines().map(crate::decode).collect::<Vec<usize>>();
    seat_ids.sort_unstable();

    let max_seat_id = *seat_ids.last().unwrap();

    let my_seat_id = seat_ids.windows(2)
        .filter(|&x| x[1] - x[0] == 2)
        .map(|x| x[1] - 1)
        .next()
        .unwrap();

    (max_seat_id, my_seat_id)
}

// ============================================================================================== //

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE_01: &'static str = include_str!("../sample01");

    #[test]
    fn part_1_decode() {
        let seats = SAMPLE_01.lines().map(crate::decode).collect::<Vec<usize>>();
        assert_eq!(seats[0], 357);
        assert_eq!(seats[1], 567);
        assert_eq!(seats[2], 119);
        assert_eq!(seats[3], 820);
    }

    #[test]
    fn part_1_sample_input() {
        let actual = SAMPLE_01.lines().map(crate::decode).max().unwrap();
        assert_eq!(actual, 820);
    }

    #[test]
    fn part_1_puzzle_input() {
        let actual = INPUT.lines().map(crate::decode).max().unwrap();
        assert_eq!(actual, 894);
    }

    #[test]
    fn part_2_puzzle_input() {
        let mut seats = INPUT.lines().map(crate::decode).collect::<Vec<usize>>();
        seats.sort_unstable();

        let actual = seats.windows(2)
            .filter(|&x| x[1] - x[0] == 2)
            .map(|x| x[1] - 1)
            .next()
            .unwrap();
        assert_eq!(actual, 579);
    }

    #[cfg(all(feature = "unstable", test))]
    mod bench {
        extern crate test;

        use crate::*;
        use test::Bencher;

        #[bench]
        fn d05p1(b: &mut Bencher) {
            b.iter(|| {
                solve()
            });
        }
    }
}