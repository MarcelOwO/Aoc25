use std::{fs::ReadDir, path::PathBuf, str::FromStr};

mod days;
mod resource;
mod solver;
mod utility;

#[allow(dead_code)]
fn main() {
    let current_day = 5;
    let mut resource_manager = resource::ResourceManager::default();
    let mut solver_manager = solver::SolverManager::default();

    register_days!(solver_manager, 1, 2, 3, 4, 5);

    utility::register_files(&mut resource_manager);

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

    //solver_manager.test(&current_day, test_data);
    solver_manager.solve(&current_day, &resource_manager);
}
