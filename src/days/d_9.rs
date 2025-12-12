use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::pipe;

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

fn get_area2(
    a: (u32, u32),
    b: (u32, u32),
    range_x: &HashMap<u32, (u32, u32)>,
    range_y: &HashMap<u32, (u32, u32)>,
) -> u64 {
    if !validate(a, b, range_x, range_y) {
        return 0;
    }

    let x = a.0.abs_diff(b.0) as u64 + 1;
    let y = a.1.abs_diff(b.1) as u64 + 1;

    x * y
}

fn get_border(a: (u32, u32), b: (u32, u32)) -> HashSet<(u32, u32)> {
    let mut points = HashSet::new();

    let x_min = min(a.0, b.0);
    let y_min = min(a.1, b.1);
    let x_max = max(a.0, b.0);
    let y_max = max(a.1, b.1);

    for x in x_min..=x_max {
        points.insert((x, y_min));
    }

    for x in x_min..=x_max {
        points.insert((x, y_max));
    }

    for y in y_min..=y_max {
        points.insert((x_min, y));
    }

    for y in y_min..=y_max {
        points.insert((x_max, y));
    }

    points
}

fn validate(
    a: (u32, u32),
    b: (u32, u32),
    range_x: &HashMap<u32, (u32, u32)>,
    range_y: &HashMap<u32, (u32, u32)>,
) -> bool {
    let rect_points = get_border(a, b);

    for (px, py) in rect_points {
        let &x_range = match range_x.get(&py) {
            Some(val) => val,
            None => continue,
        };

        if px < x_range.0 || px > x_range.1 {
            return false;
        }

        let &y_range = match range_y.get(&px) {
            Some(val) => val,
            None => continue,
        };

        if py < y_range.0 || py > y_range.1 {
            return false;
        }
    }

    true
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

impl Solver for Day9Solver {
    fn solve1(&mut self, data: &str) {
        let data = get_test_data();
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
        //let data = get_test_data();

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

        println!("getting border");

        let mut border: HashSet<(u32, u32)> = HashSet::new();
        let len = points.len();

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

        let mut max = 0;
        let total_points = len * len - 1;

        println!("building lookup");

        let mut range_x: HashMap<u32, (u32, u32)> = HashMap::new();
        let mut range_y: HashMap<u32, (u32, u32)> = HashMap::new();

        for &(x, y) in border.iter() {
            let xr = match range_x.get_mut(&y) {
                Some(val) => val,
                None => {
                    range_x.insert(y, (100000, 0));
                    range_x.get_mut(&y).unwrap()
                }
            };

            if x < xr.0 {
                xr.0 = x;
            }
            if x > xr.1 {
                xr.1 = x
            }

            let yr = match range_y.get_mut(&x) {
                Some(val) => val,
                None => {
                    range_y.insert(x, (100000, 0));
                    range_y.get_mut(&x).unwrap()
                }
            };

            if y < yr.0 {
                yr.0 = y;
            }
            if y > yr.1 {
                yr.1 = y
            }
        }

        println!("Checking");

        let mut counter = 0.0;
        let total = total_points as f64;

        for &a_point in points.iter() {
            for &b_point in points.iter() {
                let area = get_area2(a_point, b_point, &range_x, &range_y);

                println!(
                    "current max: {} with progress: {}%  of {}/{}",
                    max,
                    (counter / total),
                    counter,
                    total_points
                );

                counter += 1.0;

                if area > max {
                    max = area;
                }
            }
        }

        println!("Max: {max}");
        println!("-----------------------");
    }
}
