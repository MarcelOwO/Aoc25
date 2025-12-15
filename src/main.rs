use std::env;

mod days;
mod resource;
mod solver;
mod utility;

#[allow(dead_code)]
fn main() {
    let mut current_day: usize = 12;

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        current_day = args[1].trim().parse::<usize>().unwrap();
    }

    let mut resource_manager = resource::ResourceManager::default();

    let mut solver_manager = solver::SolverManager::default();

    // need to update this if you want more days
    utility::register_days(&mut solver_manager);

    utility::register_files(&mut resource_manager);

    solver_manager.solve(&current_day, &resource_manager);
}
