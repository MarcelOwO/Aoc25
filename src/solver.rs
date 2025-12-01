use std::collections::HashMap;

use crate::resource::ResourceManager;

pub(crate) trait Solver {
    fn solve1(&mut self, data: &str);
    fn solve2(&mut self, data: &str);
}
#[derive(Default)]
pub(crate) struct SolverManager {
    solvers: HashMap<usize, Box<dyn Solver>>,
}

impl SolverManager {
    pub(crate) fn add_solver(&mut self, key: usize, solver: Box<dyn Solver>) {
        self.solvers.insert(key, solver);
    }

    pub(crate) fn solve(&mut self, key: &usize, resource: &ResourceManager) {
        let solver = self.solvers.get_mut(key).unwrap();
        let res = resource.get_file(*key);
        solver.solve1(&res);
        solver.solve2(&res);
    }
}
