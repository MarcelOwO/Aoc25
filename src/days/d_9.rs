use std::collections::{HashSet, VecDeque};

use crate::solver::Solver;

#[derive(Default)]
pub(crate) struct Day9Solver {}

fn get_test_data() -> &'static str {
    "7,1
    11,1
    11,7
    9,7
    9,5
    2,5
    2,3
    7,3"
}

fn get_area(a: (u32, u32), b: (u32, u32)) -> u64 {
    let x = a.0.abs_diff(b.0) as u64 + 1;
    let y = a.1.abs_diff(b.1) as u64 + 1;

    x * y
}

fn get_area2(a: (u32, u32), b: (u32, u32), invalid_points: &HashSet<(u32, u32)>) -> u64 {
    if validate(a, b, invalid_points) {
        return 0;
    }

    let x = a.0.abs_diff(b.0) as u64 + 1;
    let y = a.1.abs_diff(b.1) as u64 + 1;

    x * y
}

fn validate(a: (u32, u32), b: (u32, u32), invalid: &HashSet<(u32, u32)>) -> bool {
    let mut dx = b.0 as i64 - a.0 as i64;
    let mut dy = b.1 as i64 - a.1 as i64;

    let sign_x = dx.signum();
    let abs_dx = dx.abs();

    let sign_y = dy.signum();
    let abs_dy = dy.abs();

    for i in 0..=abs_dx {
        for j in 0..=abs_dy {
            let new_x = (a.0 as i64 + i * sign_x) as u32;
            let new_y = (a.1 as i64 + j * sign_y) as u32;

            if invalid.contains(&(new_x, new_y)) {
                return true;
            }
        }
    }
    false
}

fn add_border(points: &mut HashSet<(u32, u32)>, a: (u32, u32), b: (u32, u32)) {
    let dx = b.0 as i64 - a.0 as i64;
    let dy = b.1 as i64 - a.1 as i64;

    if dy == 0 {
        let sign_x = dx.signum();
        let length = dx.abs();

        for n in 0..=length {
            let new_x = (a.0 as i64 + n * sign_x) as u32;
            points.insert((new_x, a.1));
        }
    } else if dx == 0 {
        let sign_y = dy.signum();
        let length = dy.abs();

        for n in 0..=length {
            let new_y = (a.1 as i64 + n * sign_y) as u32;
            points.insert((a.0, new_y));
        }
    } else {
        panic!("Segment is not axis-aligned!");
    }
}

fn flood_fill(
    outer: &mut HashSet<(u32, u32)>,
    start: (u32, u32),
    border: &HashSet<(u32, u32)>,
    min_x: u32,
    max_x: u32,
    min_y: u32,
    max_y: u32,
) {
    if border.contains(&start) {
        panic!("already part of the border");
    }

    let mut queue: VecDeque<(u32, u32)> = VecDeque::new();

    queue.push_back(start);
    outer.insert(start);

    while let Some((x, y)) = queue.pop_front() {
        let moves: [(i64, i64); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        for (dx, dy) in moves {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;

            if nx < min_x as i64 || nx > max_x as i64 || ny < min_y as i64 || ny > max_y as i64 {
                continue;
            }

            let x = nx as u32;
            let y = ny as u32;
            let n = (x, y);

            if border.contains(&n) {
                continue;
            }

            if outer.insert(n) {
                queue.push_back(n);
            }
        }
    }
}

impl Solver for Day9Solver {
    ///Just a simple brute force solution,
    ///it is fast enough lol
    fn solve1(&mut self, data: &str) {
        let points: Vec<(u32, u32)> = data
            .trim()
            .lines()
            .map(|x| {
                let v: Vec<u32> = x
                    .trim()
                    .split(",")
                    .map(|y| y.parse::<u32>().unwrap())
                    .collect();
                (v[0], v[1])
            })
            .collect();
        let mut max = 0;
        for &a_point in points.iter() {
            for &b_point in points.iter() {
                let area = get_area(a_point, b_point);
                if area > max {
                    max = area;
                }
            }
        }
        println!("Max: {max}");
        println!("-----------------------");
    }
    fn solve2(&mut self, data: &str) {
        println!("getting points");
        let points: Vec<(u32, u32)> = data
            .trim()
            .lines()
            .map(|x| {
                let v: Vec<u32> = x
                    .trim()
                    .split(",")
                    .map(|y| y.parse::<u32>().unwrap())
                    .collect();
                (v[0], v[1])
            })
            .collect();

        let mut max_x = 0;
        let mut max_y = 0;

        println!("getting max");

        for &(x, y) in points.iter() {
            let xu = x;
            let yu = y;

            if xu > max_x {
                max_x = xu;
            }
            if yu > max_y {
                max_y = yu;
            }
        }

        let mut border: HashSet<(u32, u32)> = HashSet::new();
        let len = points.len();
        println!("getting border");

        points.iter().for_each(|&p| {
            border.insert(p);
        });

        for n in 0..len {
            let mut next = n + 1;

            if next == len {
                next = 0
            }

            let a = points[n];
            let b = points[next];

            add_border(&mut border, a, b);
        }
        println!("border: {}", border.len());

        println!("flood");

        let max_x_safe = max_x + 1;
        let max_y_safe = max_y + 1;
        let min_x_safe = 0;
        let min_y_safe = 0;

        let start: (u32, u32) = (min_x_safe, min_y_safe);

        let mut outer = HashSet::new();

        flood_fill(
            &mut outer, start, &border, min_x_safe, max_x_safe, min_y_safe, max_y_safe,
        );

        let max_points: u128 = max_x_safe as u128 * max_y_safe as u128;

        println!(
            "total: {} outside: {}, border: {}",
            max_points,
            outer.len(),
            border.len()
        );

        println!("checking");

        let mut max = 0;

        for i in 0..len {
            let a_point = points[i];

            for j in 0..len {
                let b_point = points[j];

                //println!("Progress: {}% of {}", (i * j) / (len * len) * 100, len);

                let area = get_area2(a_point, b_point, &outer);

                if area > max {
                    max = area;
                }
            }
        }
        println!("Max: {max}");
        println!("-----------------------");
    }
}
