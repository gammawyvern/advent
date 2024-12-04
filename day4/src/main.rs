use std::io::{self, BufReader, BufRead};
use std::fs::File;

fn main() {
    let _puzzle = match load_input("./input.txt") {
        Ok(puzzle) => puzzle,
        Err(err) => {
            eprintln!("Error loading input puzzle: {}", err);
            std::process::exit(1);
        }
    };

    // TODO
}

fn load_input(path: &str) -> Result<Vec<Vec<char>>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let puzzle: Vec<Vec<char>> = reader 
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect();

    Ok(puzzle)
}
