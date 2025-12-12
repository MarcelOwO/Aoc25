use std::collections::{HashSet, VecDeque};

///
/// Placeholder of a solver to copy
///
use crate::solver::Solver;

#[derive(Default)]
pub(crate) struct Day10Solver {}

impl Day10Solver {}

fn get_test_data() -> &'static str {
    "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
}

fn bfs(start: u32, target: u32, get: &dyn Fn(&u32) -> Vec<u32>) -> Option<usize> {
    let mut queue: VecDeque<(u32, usize)> = VecDeque::new();
    let mut visited: HashSet<u32> = HashSet::new();

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((current_state, distance)) = queue.pop_front() {
        if current_state == target {
            return Some(distance);
        }

        for n in get(&current_state) {
            if !visited.contains(&n) {
                visited.insert(n);
                queue.push_back((n, distance + 1));
            }
        }
    }

    None
}

fn bfs2(
    start: Vec<u32>,
    target: Vec<u32>,
    get: &dyn Fn(&Vec<u32>) -> Vec<Vec<u32>>,
) -> Option<usize> {
    let mut queue: VecDeque<(Vec<u32>, usize)> = VecDeque::new();

    let mut visited: HashSet<Vec<u32>> = HashSet::new();

    queue.push_back((start.clone(), 0));
    visited.insert(start.clone());

    while let Some((current_state, distance)) = queue.pop_front() {
        println!("-----------------------");
        println!("{:?}", current_state);
        println!("{distance}");
        println!("target: {:?}", target);
        if current_state == target {
            return Some(distance);
        }

        if distance > 2000 {
            continue;
        }

        for n in get(&current_state) {
            if !visited.contains(&n) {
                visited.insert(n.clone());
                queue.push_back((n, distance + 1));
            }
        }
    }

    None
}

fn solve_machine(light: u32, buttons: Vec<u32>) -> u32 {
    let start = 0;
    let get = |state: &u32| -> Vec<u32> { buttons.iter().map(|mask| mask ^ state).collect() };
    bfs(start, light, &get).unwrap() as u32
}

fn solve_machine2(joltage: Vec<u32>, buttons: Vec<Vec<usize>>) -> u32 {
    let start = vec![0; joltage.len()];

    let get = |state: &Vec<u32>| -> Vec<Vec<u32>> {
        buttons
            .iter()
            .map(|mask| {
                let mut c = state.clone();
                for &n in mask {
                    c[n] += 1;
                }
                c
            })
            .collect()
    };
    bfs2(start, joltage, &get).unwrap() as u32
}

fn parse_button(data: &str) -> u32 {
    let number = &data[1..data.len() - 1];

    let mut chars = ['0'; 32];

    number
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .for_each(|f| chars[f] = '1');

    chars.reverse();

    let str: String = chars.iter().copied().collect();

    u32::from_str_radix(&str, 2).unwrap()
}

fn parse_button2(data: &str) -> Vec<usize> {
    let number = &data[1..data.len() - 1];

    let nums: Vec<usize> = number
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    nums
}

fn parse_joltage(data: &str) -> Vec<u32> {
    let number = &data[1..data.len() - 1];

    let numbers: Vec<u32> = number
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    numbers
}

fn parse_light(data: &str) -> u32 {
    let number = &data[1..data.len() - 1];

    let bin: String = number
        .chars()
        .rev()
        .map(|b| if b == '#' { '1' } else { '0' })
        .collect();

    u32::from_str_radix(&bin, 2).unwrap()
}

fn map_to_machine(data: &str) -> (u32, Vec<u32>) {
    let parts: Vec<&str> = data.split(' ').collect();

    let mut light = 0;

    let mut buttons = vec![];

    for part in parts {
        if part.starts_with('(') {
            buttons.push(parse_button(part));
        }

        if part.starts_with('[') {
            light = parse_light(part);
        }
    }

    (light, buttons)
}

fn map_to_machine2(data: &str) -> (Vec<u32>, Vec<Vec<usize>>) {
    let parts: Vec<&str> = data.split(' ').collect();

    let mut joltage = vec![];

    let mut buttons: Vec<Vec<usize>> = vec![];

    for part in parts {
        if part.starts_with('(') {
            buttons.push(parse_button2(part));
        }

        if part.starts_with('{') {
            joltage = parse_joltage(part);
        }
    }

    (joltage, buttons)
}

impl Solver for Day10Solver {
    fn solve1(&mut self, data: &str) {
        //let data = get_test_data();

        let lines: Vec<(u32, Vec<u32>)> = data
            .trim()
            .lines()
            .map(|x| map_to_machine(x.trim()))
            .collect();

        let mut counter = 0;

        for (light, buttons) in lines {
            counter += solve_machine(light, buttons);
        }
        println!("Counter: {counter}");
        println!("-----------------------");
    }
    fn solve2(&mut self, data: &str) {
        let lines: Vec<(Vec<u32>, Vec<Vec<usize>>)> = data
            .trim()
            .lines()
            .map(|x| map_to_machine2(x.trim()))
            .collect();

        let mut counter = 0;

        for (joltage, buttons) in lines {
            counter += solve_machine2(joltage, buttons);
        }
        println!("Counter: {counter}");
        println!("-----------------------");
    }
}
