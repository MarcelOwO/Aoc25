use std::{path::PathBuf, str::FromStr};

use crate::{days, resource::ResourceManager, solver::SolverManager};

pub(crate) fn register_files(manager: &mut ResourceManager) {
    let res_path = "./res";

    let mut path = PathBuf::new();

    path.push(res_path);

    let entrys = match std::fs::read_dir(path.clone()) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("filed to read dir: {}", e);
            return;
        }
    };

    for entry in entrys.into_iter() {
        let mut path_copy = path.clone();

        let entry_result = match entry {
            Ok(result) => result,
            Err(_) => continue,
        };

        let file_name = entry_result.file_name();

        path_copy.push(file_name.clone());

        //lol

        let day = match match match match file_name.into_string() {
            Ok(str) => str,
            Err(_) => {
                println!("failed to get string from ostring");
                continue;
            }
        }
        .split("_")
        .nth(1)
        {
            Some(val) => val,
            None => {
                println!("failed to get 1st element of path");
                continue;
            }
        }
        .split(".")
        .next()
        {
            Some(val) => val,
            None => {
                println!("failed to get 0th element of 1element of the string");
                continue;
            }
        }
        .parse::<usize>()
        {
            Ok(size) => size,
            Err(e) => {
                eprintln!("error in parsing path: {e}");
                continue;
            }
        };

        let path_str = match String::from_str(match path_copy.to_str() {
            Some(str) => str,
            None => {
                println!("Failed to parse string");
                continue;
            }
        }) {
            Ok(str) => str,
            Err(e) => {
                println!("error getting string from path : {}", e);
                continue;
            }
        };

        manager.add_file(day, path_str);
    }

    manager.add_file(1, "./res/day_1.txt".to_string());
}

#[macro_export]
macro_rules! register_days {
    ($manager:ident,$($day:literal),*) => {

        use paste::paste;

            $(
                paste!{
                    $manager.add_solver($day,Box::new(days::[<d_ $day>]::[<Day $day Solver>]::default()));
                }
            )*
    };
}
