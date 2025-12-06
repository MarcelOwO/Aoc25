use crate::solver::Solver;

#[derive(Default)]
pub(crate) struct Day3Solver {}

impl Solver for Day3Solver {
    fn solve1(&mut self, data: &str) {
        let mut sum = 0;

        let lines: Vec<&str> = data.split("\n").collect();

        for line in lines {
            let nums: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
            let nums_len = nums.len();

            if nums_len < 5 {
                println!("fix");
                continue;
            }

            let mut i = 0;

            let mut f = 0;

            for n in 0..nums_len - 1 {
                let current = nums[n];

                if current > f {
                    f = current;
                    i = n;
                }
            }
            let mut j = 0;

            let mut l = 0;

            for m in (i + 1)..nums_len {
                let current = nums[m];

                if current > l {
                    l = current;
                    j = m;
                }
            }

            let line_sum = f * 10 + l;
            println!("{line_sum}");

            sum += line_sum;
        }

        println!("Sum: {sum}");
    }
    fn solve2(&mut self, data: &str) {
        println!("00000000000000000000");
        let mut sum = 0;

        let lines: Vec<&str> = data.split("\n").collect();
        let k = 12;

        for line in lines {
            let nums: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();

            let nums_len = nums.len();

            if nums_len < 5 {
                println!("fix");
                continue;
            }

            let mut stack: Vec<u32> = vec![];

            let mut max_drop = nums_len - k;

            for current in nums {
                if stack.is_empty() {
                    stack.push(current);
                    continue;
                }

                while !stack.is_empty() && stack.last().unwrap() < &current {
                    if max_drop == 0 {
                        println!("max drop empty");
                        break;
                    }
                    println!("pop");
                    max_drop -= 1;
                    stack.pop();
                }

                stack.push(current);
            }

            println!("{}", stack.len());

            let mut line_sum = 0;

            for n in 0..k {
                let index = n;
                let base: u64 = 10;
                let power: u64 = base.pow((k - n - 1) as u32);
                line_sum += power * stack[index] as u64;
            }

            println!("Line:{line_sum}");

            sum += line_sum;
        }
        println!("Sum: {sum}");
    }
}
