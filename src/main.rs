use std::{fs::ReadDir, path::PathBuf, str::FromStr};

use crate::{
    days::{
        d_01::Day01Solver, d_02::Day02Solver, d_03::Day03Solver, d_04::Day04Solver,
        d_05::Day05Solver,
    },
    resource::ResourceManager,
    solver::SolverManager,
};

mod days;
mod resource;
mod solver;

fn main() {
    let current_day = 5;
    let mut resource_manager = ResourceManager::default();
    let mut solver_manager = SolverManager::default();

    register_days(&mut solver_manager);
    register_files(&mut resource_manager);

    let test_data = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    solver_manager.test(&current_day, test_data);
    //solver_manager.solve(&current_day, &resource_manager);
}

pub(crate) fn register_days(manager: &mut SolverManager) {
    let solver1 = Day01Solver::default();
    let solver2 = Day02Solver::default();
    let solver3 = Day03Solver::default();
    let solver4 = Day04Solver::default();
    let solver5 = Day05Solver::default();

    manager.add_solver(1, Box::new(solver1));
    manager.add_solver(2, Box::new(solver2));
    manager.add_solver(3, Box::new(solver3));
    manager.add_solver(4, Box::new(solver4));
    manager.add_solver(5, Box::new(solver5));
}

pub(crate) fn register_files(manager: &mut ResourceManager) {
    let res_path = "./res";

    let mut path = PathBuf::new();

    path.push(res_path);

    let entrys = match std::fs::read_dir(path.clone()) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("filed to read dir: {}", e);
            return;
        }
    };

    for entry in entrys.into_iter() {
        let mut path_copy = path.clone();

        let entry_result = match entry {
            Ok(result) => result,
            Err(_) => continue,
        };

        let file_name = entry_result.file_name();

        path_copy.push(file_name.clone());

        let day = match match match match file_name.into_string() {
            Ok(str) => str,
            Err(_) => {
                println!("failed to get string from ostring");
                continue;
            }
        }
        .split("_")
        .nth(1)
        {
            Some(val) => val,
            None => {
                println!("failed to get 1st element of path");
                continue;
            }
        }
        .split(".")
        .next()
        {
            Some(val) => val,
            None => {
                println!("failed to get 0th element of 1element of the string");
                continue;
            }
        }
        .parse::<usize>()
        {
            Ok(size) => size,
            Err(e) => {
                eprintln!("error in parsing path: {e}");
                continue;
            }
        };

        let path_str = match String::from_str(match path_copy.to_str() {
            Some(str) => str,
            None => {
                println!("Failed to parse string");
                continue;
            }
        }) {
            Ok(str) => str,
            Err(e) => {
                println!("error getting string from path : {}", e);
                continue;
            }
        };

        manager.add_file(day, path_str);
    }

    manager.add_file(1, "./res/day_1.txt".to_string());
}
