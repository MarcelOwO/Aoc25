use std::collections::HashSet;

///
/// Placeholder of a solver to copy
///
use crate::solver::Solver;

#[derive(Default)]
pub(crate) struct Day05Solver {}

impl Day05Solver {}

fn find_in_range(to_find: u64, first: u64, last: u64) -> bool {
    if last - first < 10 {
        for n in first..last + 1 {
            if n == to_find {
                println!("found: {n} in range: {first}:{last}");
                return true;
            }
        }
        return false;
    }
    let middle = (first + last) / 2;

    if to_find < middle {
        if find_in_range(to_find, first, middle) {
            return true;
        }
    } else if find_in_range(to_find, middle, last) {
        return true;
    }

    false
}

fn count_ranges(first: u64, last: u64) -> u64 {
    last - first
}

impl Solver for Day05Solver {
    fn solve1(&mut self, data: &str) {
        let mut counter = 0;
        let lines: Vec<&str> = data.trim().split("\n").collect();

        let mut ranges: Vec<(u64, u64)> = vec![];

        let mut check = false;

        for line in lines {
            if line.is_empty() {
                println!("done inserting");
                ranges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                check = true;
                continue;
            }

            if !check {
                let cleaned: Vec<&str> = line.trim().split("-").collect();

                let n_0 = cleaned[0].parse::<u64>().unwrap();
                let n_1 = cleaned[1].parse::<u64>().unwrap();

                ranges.push((n_0, n_1));
            } else {
                let id = line.trim().parse::<u64>().unwrap();

                let sub_ranges: Vec<&(u64, u64)> = ranges
                    .iter()
                    .filter(|(a, b)| id >= *a && id <= *b)
                    .collect();

                for range in sub_ranges {
                    if find_in_range(id, range.0, range.1) {
                        counter += 1;
                        break;
                    }
                }
            }
        }
        println!("{counter}");
    }
    fn solve2(&mut self, data: &str) {
        let mut ranges: Vec<(u64, u64)> = vec![];

        for line in data.trim().split("\n") {
            if line.is_empty() {
                break;
            }

            let nums: Vec<&str> = line.trim().split("-").collect();
            let n_0 = nums[0].parse::<u64>().unwrap();
            let n_1 = nums[1].parse::<u64>().unwrap();

            let n = (n_0, n_1);

            ranges.push(n);
        }

        let mut counter = 0;
        for range in ranges {
            counter += count_ranges(range.0, range.1);
        }

        println!("---------------");
        println!("Count: {counter}");
    }
}
