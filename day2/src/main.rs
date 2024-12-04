use std::env;


use std::fs;
const LOWER: i32 = 1;
const HIGHER: i32 = 3;
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let data = parse_file(filename);
    let result = count_safe_rows(&data);
    let results2 = count_safe_rows_with_removal(&data);
    println!("part 1 soln: {result}");
    println!("part 2 soln: {results2}");
}
fn parse_file(filename: &String) -> Vec<Vec<i32>> {
    let data = fs::read_to_string(filename).expect("Unable to read file");
    data.split('\n')
        .collect::<Vec<_>>()
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .into_iter()
                .map(|ele| ele.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}
fn is_monotonic(list: &Vec<i32>) -> bool {
    let mut inc_count = 0;
    let mut dec_count = 0;
    let length = list.len();
    for i in 1..length {
        let diff = list[i] - list[i - 1];
        if diff < 0 && diff.abs() <= HIGHER && diff.abs() >= LOWER {
            inc_count += 1;
        }
        if diff > 0 && diff.abs() <= HIGHER && diff.abs() >= LOWER {
            dec_count += 1;
        }
    }


    inc_count == length - 1 || dec_count == length - 1
}
fn count_safe_rows(input: &Vec<Vec<i32>>) -> i32 {
    input
        .into_iter()
        .map(|row| is_monotonic(&row))
        .filter(|b| *b)
        .count() as i32
}
fn count_safe_rows_with_removal(input: &Vec<Vec<i32>>) -> i32 {
    input
        .into_iter()
        .map(|row| {
            let mut results = vec![];
            for i in 0..row.len() {
                let filtered_row: Vec<_> = row[0..i].iter()
                    .chain(row[i+1..].iter())
                    .cloned()
                    .collect();
                results.push(is_monotonic(&filtered_row));
            }
            results.into_iter().any(|x| x)
        }).filter(|b| *b)
        .count() as i32
        
}
