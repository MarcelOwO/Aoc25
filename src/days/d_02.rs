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

    false
}

pub(crate) fn create_lps(pattern: String) -> Vec<usize> {
    let chars: Vec<char> = pattern.chars().collect();
    let n = chars.len();

    if n == 0 {
        return Vec::new();
    }

    let mut lps = vec![0; n];

    let mut len: usize = 0;
    let mut i: usize = 1;

    while i < n {
        if chars[i] == chars[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len != 0 {
            len = lps[len - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }
    lps
}

pub(crate) fn is_valid_2(num: u64) -> bool {
    let str = num.to_string();
    let str_len = str.len();
    let lps = create_lps(str);
    let overlap_len = lps[str_len - 1];
    let pot_pat_len = str_len - overlap_len;
    let has_overlap = overlap_len > 0;
    let perfect_tiling = str_len.is_multiple_of(pot_pat_len);
    has_overlap && perfect_tiling
}
/*
pub(crate) fn is_valid_2(num: u64) -> bool {
    let str = num.to_string();

    let lps = create_lps(str.clone());

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

    true
}
*/

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
                if is_valid_2(n) {
                    counter += n;
                }
            }
        }

        //66500947346
        //something like this
        println!("summ: {counter}");
    }
}
