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

    let point = (0, 7);
    let found = search_for_word(&puzzle, point, (1, -1), "XMAS");
    println!("{}", found);
}

fn search_for_word(puzzle: &Vec<Vec<char>>, point: (usize, usize), dir: (isize, isize), word: &str) -> bool {
    let next_char = match word.chars().nth(0) {
        Some(chr) => chr,
        None => return true,
    };

    // TODO replace
    if !is_point_valid(&puzzle, point) {
        return false;
    }

    if dir.0.is_negative() {
        next_row
    }

    if puzzle[point.0][point.1] != next_char {
        return false;
    }

    let next_point = (point.0 + dir.0, point.1 + dir.1);
    search_for_word(&puzzle, next_point, dir, &word[1..])
}

fn is_point_valid(puzzle: &Vec<Vec<char>>, point: (usize, usize)) -> bool {
    let (row, col) = point;

    if row >= puzzle.len() || col >= puzzle[row].len() {
        return false;
    }

    true
}
