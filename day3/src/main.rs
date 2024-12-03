use regex::Regex;
use std::fs;
use std::io;

fn main() {
    let input = load_input("./input.txt").expect("Failed to load input");

    let multi = process_multi(&input, false);
    let multi_enabled = process_multi(&input, true);
    
    println!("Multiplication: {}", multi);
    println!("Multiplication with enabled/disabled: {}", multi_enabled);
}

fn process_multi(input: &str, enabling: bool) -> usize {
    let command_pattern = r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)";
    let re = Regex::new(command_pattern).unwrap();

    let mut enabled = true;
    let mut sum = 0;

    for inst in re.captures_iter(&input) {
        if let Some(_) = inst.get(1) {
            let num1: usize = inst[1].parse().unwrap();
            let num2: usize = inst[2].parse().unwrap();
            
            if enabled {
                sum += num1 * num2;
            }
        } else if enabling && inst[0] == *"do()" {
            enabled = true;
        } else if enabling && inst[0] == *"don't()" {
            enabled = false;
        }

    }

    sum
}

fn load_input(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)  
}
