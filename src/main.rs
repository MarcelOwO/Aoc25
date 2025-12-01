use crate::{days::d_01::Day01Solver, resource::ResourceManager, solver::SolverManager};

mod days;
mod resource;
mod solver;

fn main() {
    let current_day = 1;
    let mut resource_manager = ResourceManager::default();
    let mut solver_manager = SolverManager::default();

    register_days(&mut solver_manager);
    register_files(&mut resource_manager);

    solver_manager.solve(&current_day, &resource_manager);
}

pub(crate) fn register_days(manager: &mut SolverManager) {
    let solver = Day01Solver::default();
    manager.add_solver(1, Box::new(solver));
}

pub(crate) fn register_files(manager: &mut ResourceManager) {
    manager.add_file(1, "../res/day_1.txt".to_string());
}
