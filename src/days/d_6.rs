use crate::solver::Solver;

#[derive(Default)]
pub(crate) struct Day6Solver {}

fn apply_op(stack: &mut Vec<u64>, op: bool) -> u64 {
    println!("applying {op}");

    let mut local = 0;

    loop {
        let val = match stack.pop() {
            Some(val) => val,
            None => {
                return local;
            }
        };

        if local == 0 {
            local = val;
            continue;
        }

        if (op) {
            local *= val;
        } else {
            local += val;
        }
    }
}

impl Solver for Day6Solver {
    fn solve1(&mut self, data: &str) {
        let mut sum = 0;

        let rows: Vec<&str> = data.trim().split('\n').collect();

        let mut stack_list: Vec<Vec<u64>> = vec![];

        let mut len = 0;

        for n in 0..rows.len() {
            let row = rows[n];
            println!("{row}");
            let row_items: Vec<&str> = row
                .trim()
                .split(" ")
                .filter(|c| !c.is_empty() && !c.trim().is_empty())
                .collect();

            if n == 0 {
                len = row_items.len();
                stack_list = vec![vec![]; len];
            }

            if n == rows.len() - 1 {
                for m in 0..len {
                    sum += apply_op(&mut stack_list[m], row_items[m] == "*");
                }
            } else {
                for m in 0..len {
                    println!("{m}: {}", row_items[m]);

                    let current = match row_items[m].parse::<u64>() {
                        Ok(val) => val,
                        Err(_) => {
                            println!("tried to parse: {}", row_items[m]);
                            return;
                        }
                    };
                    stack_list[m].push(current);
                }
            }
        }
        println!("Sum: {sum}");
    }
    fn solve2(&mut self, data: &str) {}
}
