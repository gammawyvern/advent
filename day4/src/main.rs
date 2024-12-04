use std::io::{self, BufReader, BufRead};
use std::fs::File;

fn main() {
    let puzzle = match load_input("./input.txt") {
        Ok(puzzle) => puzzle,
        Err(err) => {
            eprintln!("Error loading input puzzle: {}", err);
            std::process::exit(1);
        }
    };

    let word = "XMAS";
    search_puzzle(&puzzle, &word);
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

// TODO Assuming properly sized puzzle and word 
fn search_puzzle(puzzle: &Vec<Vec<char>>, word: &str) {
    let rows = puzzle.len();
    let cols = puzzle[0].len(); 

    for row in 0..rows {
        for col in 0..cols {
            // if puzzle[row][col] == word.chars().nth(0) {
            //     println!("Potential word found");
            // }
        }
    }
}

fn search_for_word(puzzle: &Vec<Vec<char>>, point: (usize, usize), dir: (usize, usize), word: &str) -> bool {
    let next_char = match word.chars().nth(0) {
        Some(chr) => chr,
        None => return false,
    };

    if !is_point_valid(&puzzle, point) {
        return false;
    }

    if puzzle[point.0][point.1] != next_char {
        return false;
    }

    true
}

fn is_point_valid(puzzle: &Vec<Vec<char>>, point: (usize, usize)) -> bool {
    let (row, col) = point;

    if row >= puzzle.len() || col >= puzzle[row].len() {
        return false;
    }

    true
}
