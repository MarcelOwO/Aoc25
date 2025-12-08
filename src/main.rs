use std::{fs::ReadDir, path::PathBuf, str::FromStr};

mod days;
mod resource;
mod solver;
mod utility;

#[allow(dead_code)]
fn main() {
    let current_day = 7;

    let mut resource_manager = resource::ResourceManager::default();

    let mut solver_manager = solver::SolverManager::default();

    register_days!(solver_manager, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);

    utility::register_files(&mut resource_manager);

    solver_manager.solve(&current_day, &resource_manager);
}
