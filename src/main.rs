use std::env;
use std::fs;

pub mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();

    let problem = &args[1];
    let input_file_path = &args[2];

    let input = fs::read_to_string(input_file_path).unwrap();

    let result = match problem.as_str() {
        "2.1" => solutions::problem_2::solve_part_1(&input),
        "2.2" => solutions::problem_2::solve_part_2(&input),
        _ => {
            panic!("unsupported problem {}", problem);
        }
    };

    println!("The answer is {}", result);
}
