use std::collections::{HashMap, HashSet};

use crate::solver::Solver;

#[derive(Default)]
pub(crate) struct Day11Solver {}

fn get_test_data() -> &'static str {
    "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"
}

fn get_test_data2() -> &'static str {
    "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
}

fn map_line(
    line: &str,
    lookup: &mut HashMap<String, usize>,
    paths: &mut HashMap<usize, Vec<usize>>,
) {
    let mut a = line.trim().split(':');

    let start = String::from(a.next().unwrap());

    let targets: Vec<String> = a
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.trim().is_empty())
        .map(String::from)
        .collect();
    if !lookup.contains_key(&start.clone()) {
        lookup.insert(start.clone(), lookup.len());
    }

    let &key = lookup.get(&start).unwrap();

    let key_val = match paths.get_mut(&key) {
        Some(val) => val,
        None => {
            paths.insert(key, vec![]);
            paths.get_mut(&key).unwrap()
        }
    };

    for target in targets {
        if !lookup.contains_key(&target) {
            lookup.insert(target.clone(), lookup.len());
        }

        let &val = lookup.get(&target).unwrap();

        key_val.push(val);
    }
}

fn bfs(paths: &HashMap<usize, Vec<usize>>, start: usize, target: usize) -> u32 {
    let mut queue = vec![];
    queue.push(start);

    let mut counter: u32 = 0;

    while let Some(node) = queue.pop() {
        if node == target {
            counter += 1;
            continue;
        }

        if let Some(targets) = paths.get(&node) {
            queue.extend(targets);
        };
    }

    counter
}

fn dfs(
    paths: &HashMap<usize, Vec<usize>>,
    cache: &mut HashMap<(usize, bool, bool), u64>,
    current: usize,
    target: usize,
    fft: usize,
    dac: usize,
    found_f: bool,
    found_d: bool,
) -> u64 {
    let new_found_f = found_f || (current == fft);
    let new_found_d = found_d || (current == dac);

    let state_key = (current, new_found_f, new_found_d);

    if let Some(&count) = cache.get(&state_key) {
        return count;
    }

    if current == target {
        return if new_found_f && new_found_d { 1 } else { 0 };
    }

    let mut total_paths = 0;

    if let Some(neighbors) = paths.get(&current) {
        for &neighbor in neighbors {
            total_paths += dfs(
                paths,
                cache,
                neighbor,
                target,
                fft,
                dac,
                new_found_f,
                new_found_d,
            );
        }
    }

    cache.insert(state_key, total_paths);
    total_paths
}

impl Solver for Day11Solver {
    fn solve1(&mut self, data: &str) {
        let mut paths: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut lookup: HashMap<String, usize> = HashMap::new();

        data.trim()
            .lines()
            .for_each(|x| map_line(x, &mut lookup, &mut paths));

        let total = bfs(
            &paths,
            *lookup.get("you").unwrap(),
            *lookup.get("out").unwrap(),
        );

        println!("Total: {}", total);
    }

    fn solve2(&mut self, data: &str) {
        println!("-----------------------");
        //let data = get_test_data2();

        let mut paths: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut lookup: HashMap<String, usize> = HashMap::new();

        data.trim()
            .lines()
            .for_each(|x| map_line(x, &mut lookup, &mut paths));

        let mut cache: HashMap<(usize, bool, bool), u64> = HashMap::new();
        let total = dfs(
            &paths,
            &mut cache,
            *lookup.get("svr").unwrap(),
            *lookup.get("out").unwrap(),
            *lookup.get("fft").unwrap(),
            *lookup.get("dac").unwrap(),
            false,
            false,
        );

        println!("Total: {}", total);
    }
}
