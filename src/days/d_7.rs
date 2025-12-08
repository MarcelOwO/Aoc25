use std::collections::{HashMap, HashSet};

use crate::solver::Solver;

#[derive(Default)]
pub(crate) struct Day7Solver {}

impl Day7Solver {}

fn get_test_data() -> String {
    ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
        .to_string()
}

fn process_line(line: &str, beams: HashSet<usize>) -> (i32, HashSet<usize>) {
    let chars: Vec<char> = line.chars().collect();
    let mut out = HashSet::new();
    let mut count = 0;

    for n in beams {
        let char = chars[n];
        if char == '.' {
            out.insert(n);
            continue;
        }
        if char == '^' {
            count += 1;
            out.insert(n - 1);
            out.insert(n + 1);
            continue;
        }
    }

    (count, out)
}

fn process_line2(line: &str, beams: &mut HashMap<usize, i64>) {
    let chars: Vec<char> = line.chars().collect();

    let mut new: Vec<(usize, i64)> = vec![];

    for (k, v) in beams.iter() {
        if chars[*k] == '.' {
            continue;
        }
        new.push((*k, *v * -1));
        new.push((k + 1, *v));
        new.push((k - 1, *v));
    }

    for p in new {
        let current = match beams.get(&p.0) {
            Some(val) => *val,
            None => 0,
        };
        let new_val = current + p.1;
        beams.insert(p.0, new_val);
    }
}

impl Solver for Day7Solver {
    fn solve1(&mut self, data: &str) {
        //let data = get_test_data();
        let mut beams = HashSet::new();

        let lines: Vec<&str> = data.trim().split("\n").map(|x| x.trim()).collect();

        let line1: Vec<char> = lines[0].chars().collect();

        for n in 0..line1.len() {
            if line1[n] == 'S' {
                beams.insert(n);
                break;
            }
        }
        let mut total_count = 0;
        for n in 1..lines.len() {
            let (count, temp) = process_line(lines[n], beams);
            beams = temp;
            total_count += count;
        }

        println!("Count: {}", total_count);
    }
    fn solve2(&mut self, data: &str) {
        //let data = get_test_data();

        let lines: Vec<&str> = data.trim().split("\n").map(|x| x.trim()).collect();

        let line1: Vec<char> = lines[0].chars().collect();

        let mut beams: HashMap<usize, i64> = HashMap::new();

        for n in 0..line1.len() {
            if line1[n] == 'S' {
                let current = match beams.get(&n) {
                    Some(val) => *val,
                    None => 0,
                };

                beams.insert(n, current + 1);

                break;
            }
        }

        let mut total_count: u64 = 0;

        for n in 1..lines.len() {
            process_line2(lines[n], &mut beams);
            beams.iter().for_each(|x| {
                print!("{}:{} |", x.0, x.1);
            });

            print!("\n")
        }
        beams.values().for_each(|x| total_count += *x as u64);

        println!("otherCount: {}", beams.len());
        println!("Count: {}", total_count);
    }
}
