use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let reports = load_reports("./input.txt");
    let safety = safety_report(&reports);
    let dampened_safety = dampen_report(&reports);

    println!("Strict Safety Report: {}", safety);
    println!("Dampened Safety Report: {}", dampened_safety);
}

fn load_reports(path: &str) -> Vec<Vec<usize>> {
    let report_file = File::open(path).expect("Failed to open input file");    
    let reader = BufReader::new(report_file);

    reader.lines()
        .map(|line| {
            let line = line.expect("Failed to read a report");

            line
                .split_whitespace()
                .map(|level| level.parse::<usize>().expect("Failed to parse level value"))
                .collect()
        })
        .collect()
}

fn safety_report(reports: &[Vec<usize>]) -> usize {
    reports
        .iter()
        .filter(|report| {
            let inc = report.iter().is_sorted();
            let dec = report.iter().rev().is_sorted();

            let safe = report
                .windows(2)
                .all(|levels| {
                    let diff = levels[0].abs_diff(levels[1]);
                    diff > 0 && diff <= 3
                });

            (inc || dec) && safe
        })
        .count()
}

/// Exhaustive report, not very efficient
fn dampen_report(reports: &[Vec<usize>]) -> usize {
    reports 
        .iter()
        .filter(|report| {
            let mut all_possible = Vec::new(); 

            for l in 0..report.len() {
                let new_report = [
                    &report[0..l],
                    &report[l+1..]
                ].concat();

                all_possible.push(new_report);
            }

            safety_report(&all_possible) > 0
        })
        .count()
}
