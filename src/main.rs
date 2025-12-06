use std::{fs::ReadDir, path::PathBuf, str::FromStr};

mod days;
mod resource;
mod solver;
mod utility;

#[allow(dead_code)]
fn main() {
    let current_day = 6;
    let mut resource_manager = resource::ResourceManager::default();
    let mut solver_manager = solver::SolverManager::default();

    register_days!(solver_manager, 1, 2, 3, 4, 5, 6);

    utility::register_files(&mut resource_manager);

    let test_data = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";

    //solver_manager.test(&current_day, test_data);
    solver_manager.solve(&current_day, &resource_manager);
}
