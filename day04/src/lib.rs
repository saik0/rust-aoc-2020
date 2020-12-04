#![cfg_attr(feature = "unstable", feature(test))]

const INPUT: &'static str = include_str!("../input");

// =====

fn parse(input: &str) -> Vec<[Option<&str>; 7]> {
    input.split("\n\n")
        .map(|line| {
            line.split(&[' ', '\n'][..])
                .filter_map(|entry| {
                    let mut split = entry.split(':');
                    split.next().and_then(|key| {
                        split.next().map(|value| (key, value))
                    })
                })
                .fold(Default::default(), |mut acc: [Option<&str>; 7], (key, value)| {
                    if let Ok(i) = ["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"]
                        .binary_search(&key) {
                        acc[i] = Some(value)
                    }
                    acc
                })
        }).collect()
}

fn validate_part_1(passport: &[Option<&str>; 7]) -> bool {
    passport.iter().all(Option::is_some)
}

fn validate_part_2(passport: &[Option<&str>; 7]) -> bool {
    passport[0].and_then(|value| value.parse::<usize>().ok())
        .filter(|x| (1920..=2002).contains(x))
        .is_some() &&

        // ecl
        passport[1].and_then(|value| {
            ["amb", "blu", "brn", "grn", "gry", "hzl", "oth"]
                .binary_search(&value)
                .ok()
        }).is_some() &&

        // eyr
        passport[2].and_then(|value| value.parse::<usize>().ok())
            .filter(|x| (2010..=2030).contains(x))
            .is_some() &&

        // "hcl"
        passport[3].filter(|value| {
            let bytes = value.as_bytes();
            bytes.len() == 7 &&
                bytes[0] == '#' as u8 &&
                bytes[1..].iter().all(|&b| b.is_ascii_hexdigit())
        }).is_some() &&

        // hgt
        passport[4].and_then(|value| {
            let (height, unit) = value.split_at(value.len() - 2);
            height.parse::<usize>().ok()
                .filter(|h| {
                    match unit {
                        "cm" => (150..=193).contains(h),
                        "in" => (59..=76).contains(h),
                        _ => false
                    }
                })
        }).is_some() &&

        // iyr
        passport[5].and_then(|value| value.parse::<usize>().ok())
            .filter(|x| (2010..=2020).contains(x))
            .is_some() &&

        // pid
        passport[6].filter(|value| {
            value.as_bytes().len() == 9 && value.chars().all(|c| c.is_ascii_digit())
        }).is_some()
}

fn count_valid(passports: &[[Option<&str>; 7]], validator: impl Fn(&[Option<&str>; 7]) -> bool) -> usize {
    passports.iter()
        .filter(|&passport| {
            validator(passport)
        })
        .count()
}

pub fn solve() -> (usize, usize) {
    let passports = &parse(INPUT);
    (count_valid(passports, crate::validate_part_1), count_valid(passports, crate::validate_part_2))
}

// ============================================================================================== //

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE_01: &'static str = include_str!("../sample01");
    const INVALID: &'static str = include_str!("../invalid");
    const VALID: &'static str = include_str!("../valid");

    fn solve_part_1(input: &str) -> usize {
        let passports = &parse(input);
        count_valid(passports, crate::validate_part_1)
    }

    fn solve_part_2(input: &str) -> usize {
        let passports = &parse(input);
        count_valid(passports, crate::validate_part_2)
    }

    #[test]
    fn part_1_sample_input() {
        let actual = solve_part_1(SAMPLE_01);
        assert_eq!(actual, 2);
    }

    #[test]
    fn part_1_puzzle_input() {
        let actual = solve_part_1(INPUT);
        assert_eq!(actual, 237);
    }

    #[test]
    fn part_2_invalid_passports() {
        let actual = solve_part_2(INVALID);
        assert_eq!(actual, 0);
    }

    #[test]
    fn part_2_valid_passports() {
        let actual = solve_part_2(VALID);
        assert_eq!(actual, 4);
    }

    #[test]
    fn part_2_sample_input() {
        let actual = solve_part_2(SAMPLE_01);
        assert_eq!(actual, 2);
    }

    #[test]
    fn part_2_puzzle_input() {
        let actual = solve_part_2(INPUT);
        assert_eq!(actual, 172);
    }

    #[cfg(all(feature = "unstable", test))]
    mod bench {
        extern crate test;

        use crate::*;
        use crate::tests::*;
        use test::Bencher;

        #[bench]
        fn d04_parse_input(b: &mut Bencher) {
            b.iter(|| {
                parse(INPUT);
            });
        }

        #[bench]
        fn d04p1(b: &mut Bencher) {
            b.iter(|| {
                solve_part_1(INPUT);
            });
        }

        #[bench]
        fn d04p2(b: &mut Bencher) {
            b.iter(|| {
                solve_part_2(INPUT);
            });
        }

        #[bench]
        fn d04_solve_both(b: &mut Bencher) {
            b.iter(|| {
                solve();
            });
        }

        #[bench]
        fn d04_validate_p1(b: &mut Bencher) {
            let passports = &parse(INPUT);
            b.iter(|| {
                count_valid(passports, crate::validate_part_1);
            });
        }

        #[bench]
        fn d04_validate_p2(b: &mut Bencher) {
            let passports = &parse(INPUT);
            b.iter(|| {
                count_valid(passports, crate::validate_part_2);
            });
        }
    }
}