use std::str::FromStr;

use crate::solver::Solver;

#[derive(Default)]
pub(crate) struct Day02Solver {}

pub(crate) fn is_valid_1(num: u64) -> bool {
    let str = num.to_string();

    let length = str.len();

    if length % 2 == 1 {
        return true;
    }

    let halve = length / 2;

    for n in 0..halve {
        let char_n = str.clone().chars().nth(n);
        let char_len = str.clone().chars().nth(n + halve);
        if char_n != char_len {
            return true;
        }
    }

    return false;
}

pub(crate) fn is_valid_2(num: u64) -> bool {
    let str = num.to_string();

    'outer: for n in 0..str.len() / 2 {
        let pattern = String::from_str(str.split_at(n + 1).0).unwrap();

        let length_pat = n;

        let mut i = 0;

        for n in 0..str.len() {
            let current_pattern = pattern.clone().chars().nth(i).unwrap();

            let current = str.chars().nth(n).unwrap();

            if current != current_pattern {
                continue 'outer;
            }

            i += 1;

            if i > length_pat {
                i = 0;
            }
        }

        if i != 0 {
            continue 'outer;
        }

        return false;
    }

    return true;
}

impl Solver for Day02Solver {
    fn solve1(&mut self, data: &str) {
        let mut counter = 0;
        for range in data.split(",") {
            let mut split = range.split("-");

            let start_str = split.next().unwrap().trim();

            let start = start_str.parse::<u64>().unwrap();

            let end_str = split.next().unwrap().trim();

            let end = end_str.parse::<u64>().unwrap();

            for n in start..end + 1 {
                if !is_valid_1(n) {
                    counter += n;
                }
            }
        }
        println!("summ: {counter}");
    }

    fn solve2(&mut self, data: &str) {
        let mut counter = 0;
        for range in data.split(",") {
            let mut split = range.split("-");

            let start_str = split.next().unwrap().trim();

            let start = start_str.parse::<u64>().unwrap();

            let end_str = split.next().unwrap().trim();

            let end = end_str.parse::<u64>().unwrap();

            for n in start..end + 1 {
                if !is_valid_2(n) {
                    counter += n;
                }
            }
        }
        println!("summ: {counter}");
    }
}
