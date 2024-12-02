use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let reports = load_reports("./input.txt");
    let safety = safety_report(&reports);

    reports
        .iter()
        .for_each(|report| {
            report.iter();
        });
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
        .map(|report| {

        })
        .filter(|safe| safe)
        .count()
}
