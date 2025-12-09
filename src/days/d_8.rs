use std::collections::{HashMap, HashSet, VecDeque};

use crate::solver::Solver;

use kiddo::{KdTree, SquaredEuclidean};
use petgraph::{data::Build, graph::UnGraph};
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

#[derive(Default)]
pub(crate) struct Day8Solver {}

impl Day8Solver {}

fn get_test_data() -> &'static str {
    "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
}

fn str_to_points(data: &str) -> Vec<[f64; 3]> {
    data.trim()
        .lines()
        .map(|s| {
            let v: Vec<f64> = s
                .trim()
                .split(',')
                .map(|x| x.parse::<f64>().unwrap())
                .collect();
            [v[0], v[1], v[2]]
        })
        .collect()
}
fn points_to_edge(points: Vec<[f64; 3]>) -> Vec<(f64, usize, usize)> {
    let tree: KdTree<_, 3> = (&points).into();

    points
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            let v: Vec<(f64, usize, usize)> = tree
                .nearest_n::<SquaredEuclidean>(x, 11)
                .iter()
                .filter(|neighbour| neighbour.distance > 0.0)
                .map(|y| (y.distance, i, y.item as usize))
                .take(10)
                .collect();
            v
        })
        .collect()
}

impl Solver for Day8Solver {
    fn solve1(&mut self, data: &str) {
        let data = get_test_data();

        let points = str_to_points(data);

        let mut val = points_to_edge(points);

        val.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        val.dedup_by(|a, b| (a.0 - b.0).abs() < 0.00001 && a.1 ^ a.2 ^ b.1 ^ b.2 == 0);

        let mut adjacency: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut all_unique_nodes = HashSet::new();

        for v in val.iter().take(10) {
            let (a, b) = (v.1, v.2);

            adjacency.entry(a).or_default().push(b);
            adjacency.entry(b).or_default().push(a);

            all_unique_nodes.insert(a);
            all_unique_nodes.insert(b);
        }

        let mut components_by_id: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut current_component_id = 0;
        let mut visited = HashSet::new();

        for start_node in all_unique_nodes.into_iter() {
            if visited.contains(&start_node) {
                continue;
            }

            let mut queue = VecDeque::new();
            queue.push_back(start_node);
            visited.insert(start_node);

            let mut current_node_list = Vec::new();

            while let Some(node) = queue.pop_front() {
                current_node_list.push(node);

                if let Some(neighbors) = adjacency.get(&node) {
                    for &neighbor in neighbors {
                        if visited.insert(neighbor) {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }

            components_by_id.insert(current_component_id, current_node_list);

            current_component_id += 1;
        }

        let mut circuits: Vec<Vec<usize>> = components_by_id.into_values().collect();

        circuits.sort_by(|a, b| b.len().partial_cmp(&a.len()).unwrap());
        let mut counter: u64 = 1;

        for n in 0..3 {
            counter *= circuits[n].len() as u64;
        }

        println!("{counter}");
        println!("----------------");
    }

    fn solve2(&mut self, data: &str) {
        //let data = get_test_data();

        let points = str_to_points(data);

        let points_len = points.len();

        let mut val = points_to_edge(points.clone());

        val.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        val.dedup_by(|a, b| (a.0 - b.0).abs() < 0.00001 && a.1 ^ a.2 ^ b.1 ^ b.2 == 0);

        let edges: Vec<(usize, usize)> = val.iter().map(|v| (v.1, v.2)).collect();

        let mut uf = QuickUnionUf::<UnionBySize>::new(points_len);

        let mut done = false;
        let mut last = (0, 0);

        'outer: for (a, b) in edges {
            uf.union(a, b);
            last = (a, b);

            let find = uf.find(a);

            for j in 0..points_len {
                let inner = uf.find(j);

                if inner != find {
                    continue 'outer;
                }
            }

            done = true;
            break 'outer;
        }

        if done {
            let mut counter: f64 = 0.0;

            let a = points[last.0];
            let b = points[last.1];

            counter = a[0] as f64 * b[0] as f64;

            println!("{counter}");
        } else {
            println!("Something wrong");
        }
    }
}
