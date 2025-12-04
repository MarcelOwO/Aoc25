use crate::solver::Solver;
use array2d::Array2D;

#[derive(Default)]
pub(crate) struct Day04Solver {}

impl Day04Solver {}

fn check_surrounding(x: usize, y: usize, length: i32, height: i32, matrix: &Vec<Vec<bool>>) -> i32 {
    let mut counter = 0;

    //good enough
    let vals: Vec<(i32, i32)> = vec![
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    //recalculate but whatever
    let max_r = length;
    let max_c = height;

    for (vX, vY) in vals {
        let c_i = x as i32 - vX;
        let r_i = y as i32 - vY;

        if c_i < 0 || c_i >= max_c {
            continue;
        }

        if r_i < 0 || r_i >= max_r {
            continue;
        }

        let column = &matrix[c_i as usize];

        let val = column[r_i as usize];

        if val {
            counter += 1;
        }
    }

    counter
}

impl Solver for Day04Solver {
    fn solve1(&mut self, data: &str) {
        let mut counter = 0;

        //good enough
        let lines: Vec<Vec<bool>> = data
            .trim()
            .split("\n")
            .map(|x| {
                x.chars()
                    .filter(|f| !f.is_whitespace())
                    .map(|c| c == '@')
                    .collect()
            })
            .collect();

        let length = lines[0].len();
        let height = lines.len();

        for x in 0..length {
            for y in 0..height {
                if lines[x][y] && check_surrounding(x, y, length as i32, height as i32, &lines) < 4
                {
                    counter += 1;
                }
            }
        }

        println!("Count: {counter}");
    }
    fn solve2(&mut self, data: &str) {
        println!("......................");
        let mut counter = 0;

        let mut total_rolls = 0;

        //good enough
        let mut lines: Vec<Vec<bool>> = data
            .trim()
            .split("\n")
            .map(|x| {
                x.chars()
                    .filter(|f| !f.is_whitespace())
                    .map(|c| {
                        if c == '@' {
                            total_rolls += 1;
                            return true;
                        }

                        false
                    })
                    .collect()
            })
            .collect();
        println!("Total: {total_rolls}");

        let length = lines[0].len();
        let height = lines.len();

        loop {
            println!("loop");
            let mut local_counter = 0;

            for x in 0..length {
                for y in 0..height {
                    if lines[x][y]
                        && check_surrounding(x, y, length as i32, height as i32, &lines) < 4
                    {
                        local_counter += 1;
                        lines[x][y] = false;
                        counter += 1;
                    }
                }
            }

            if local_counter == 0 || total_rolls == counter {
                break;
            }
        }
        println!("Count: {counter}");
    }
}
