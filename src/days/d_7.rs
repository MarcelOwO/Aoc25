use std::collections::HashSet;

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

fn process_line2(line: &str, beams: &mut Vec<usize>) -> u32 {
    let chars: Vec<char> = line.chars().collect();

    let mut count = 0;

    let mut new = vec![];

    for n in 0..beams.len() {
        let beam = beams[n];

        let char = chars[beam];

        if char == '.' {
            continue;
        }

        if char == '^' {
            count += 1;
            new.push(beam - 1);
            new.push(beam + 1);

            beams.remove(n);
            continue;
        }
    }

    beams.append(&mut new);

    count
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
        let mut beams = vec![];

        let lines: Vec<&str> = data.trim().split("\n").map(|x| x.trim()).collect();

        let line1: Vec<char> = lines[0].chars().collect();

        for n in 0..line1.len() {
            if line1[n] == 'S' {
                beams.push(n);
                break;
            }
        }
        let mut total_count: u64 = 0;

        for n in 1..lines.len() {
            let count = process_line2(lines[n], &mut beams);

            total_count += count as u64;

            println!("Progress: {}%", n as f32 / lines.len() as f32 * 100.0);
        }

        println!("otherCount: {}", beams.len());
        println!("Count: {}", total_count);
    }
}
