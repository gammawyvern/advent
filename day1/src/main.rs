use std::io::{self, BufRead};
use std::fs::File;
use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("Expected input file arg".to_string());
    }

    let (left, right) = load_lists(&args[1]);

    let dist = distance(&left, &right);
    let sim = similarity(&left, &right);

    println!("Distance: {}", dist);
    println!("Similarity: {}", sim);

    Ok(())
}

fn distance(left: &[usize], right: &[usize]) -> usize {
    let mut left = Vec::from(left);
    let mut right = Vec::from(right);

    left.sort();
    right.sort();

    let pair_iter = left.iter().zip(right.iter());

    pair_iter.map(|(l, r)| l.abs_diff(*r)).sum()
} 

fn similarity(left: &[usize], right: &[usize]) -> usize {
    left
        .iter()
        .map(|l_id| {
            let freq = right
                .iter()
                .filter(|r_id| *r_id == l_id)
                .count();

            l_id * freq
        })
        .sum()
}

fn load_lists(path: &str) -> (Vec<usize>, Vec<usize>){
    let file = File::open(&path).expect("Failed to load");
    let reader = io::BufReader::new(file);

    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to load line");
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() != 2 {
            panic!("Line did not contain 2 values: {}", line);
        }

        let l = parts[0].parse::<usize>().expect("Failed to parse number");
        let r = parts[1].parse::<usize>().expect("Failed to parse number");

        left.push(l);
        right.push(r);
    }

    (left, right)
}
