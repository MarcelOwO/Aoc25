use crate::solver::Solver;

#[derive(Default)]
pub(crate) struct Day6Solver {}

fn get_test_data() -> &'static str {
    "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
}

fn apply_op(stack: &mut Vec<u64>, op: bool) -> u64 {
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
                    let current = match row_items[m].parse::<u64>() {
                        Ok(val) => val,
                        Err(_) => {
                            return;
                        }
                    };
                    stack_list[m].push(current);
                }
            }
        }
        println!("Sum: {sum}");
    }

    fn solve2(&mut self, data: &str) {
        //let data = get_test_data();
        let lines: Vec<Vec<char>> = data
            .trim()
            .split("\n")
            .map(|x| x.chars().collect())
            .collect();

        let mut op: bool = false;

        let mut counter = 0;

        let mut word_total: u64 = 0;
        let rows = lines.len();

        for n in 0..lines[0].len() {
            let op_char = match lines.get(rows - 1).unwrap().get(n) {
                Some(val) => *val,
                None => ' ',
            };

            // reset

            if op_char == '*' {
                println!("{op} word: {word_total}");
                op = true;
                counter += word_total;

                println!("--------------------");
                word_total = 1; //mult base
            } else if op_char == '+' {
                println!("{op} word: {word_total}");
                op = false;

                println!("--------------------");
                counter += word_total;
                word_total = 0; //add base
            }

            let mut num = String::new();

            for m in 0..lines.len() - 1 {
                let num_char = match match lines.get(m) {
                    Some(val) => val,
                    None => break,
                }
                .get(n)
                {
                    Some(val) => *val,
                    None => '0',
                };

                num.push(num_char);
            }
            println!("{num}");
            let mut line_total = num.trim().parse().unwrap_or_default();
            println!("line: {line_total}");

            if op {
                if line_total == 0 {
                    line_total = 1;
                }
                word_total *= line_total;
            } else {
                word_total += line_total;
            }

            println!("current: {word_total}");
        }

        // add remainder
        counter += word_total;

        println!("Total: {counter}");
    }
}
