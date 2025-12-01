use crate::solver::Solver;

#[derive(Default)]
pub(crate) struct Day01Solver {}

impl Day01Solver {}

impl Solver for Day01Solver {
    fn solve1(&mut self, data: &str) {
        let mut counter = 0;
        let mut dial = 50;
        let lines = data.split("\n");
        for line in lines {
            let trimmed = line.trim();

            let sign = trimmed.starts_with('R');

            let multiplier = match sign {
                true => 1,
                false => -1,
            };

            let distance_str = trimmed
                .split_at_checked(1)
                .map(|touple| touple.1)
                .unwrap_or("0");

            let distance = multiplier * distance_str.parse::<i32>().unwrap();

            let mut new_dial = (dial + distance) % 100;

            if new_dial < 0 {
                new_dial += 100;
            }

            if new_dial == 0 || new_dial == 100 {
                counter += 1;
            }

            dial = new_dial;
        }

        println!("Clicks: {counter}");
        println!("////////////////////////////////////////");
    }
    fn solve2(&mut self, data: &str) {
        let mut counter = 0;
        let mut dial = 50;
        let lines = data.split("\n");
        for line in lines {
            let trimmed = line.trim();
            let sign = trimmed.starts_with('R');
            let multiplier = match sign {
                true => 1,
                false => -1,
            };
            let distance_str = trimmed
                .split_at_checked(1)
                .map(|touple| touple.1)
                .unwrap_or("0");

            let distance = multiplier * distance_str.parse::<i32>().unwrap();

            let mut new_dial = dial + distance;
            println!("new: {new_dial}");

            loop {
                if dial == 0 && new_dial < 0 && new_dial > -100 {
                    new_dial += 100;
                    continue;
                }

                if dial == 100 && new_dial > 100 && new_dial < 200 {
                    new_dial -= 100;
                    continue;
                }

                if new_dial < 0 {
                    new_dial += 100;
                    counter += 1;
                    println!("over click");
                    continue;
                }

                if new_dial > 100 {
                    new_dial -= 100;
                    counter += 1;
                    println!("over click");
                    continue;
                }

                break;
            }

            if new_dial < 0 {
                new_dial += 100;
            }

            if new_dial == 0 || new_dial == 100 {
                counter += 1;
                println!("zero click");
            }

            dial = new_dial;

            println!("Dial: {dial}");
            println!("...........");
        }

        println!("Clicks: {counter}");
        println!("////////////////////////////////////////");
    }
}
