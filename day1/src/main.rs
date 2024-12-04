use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    let (left,right) = parse_file(filepath);
    let result: i32 = calculate_distance(&left,&right);
    let part2 = calculate_similarity(&left, &right);
    println!("{result}");
    println!("{part2}");
    
    
}
fn parse_file(filepath:&String)-> (Vec<i32>,Vec<i32>){
   

    let reader = BufReader::new(File::open(filepath).expect("Cannot open file.txt"));
    
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in reader.lines() {
        let terms: Vec<_> = line.unwrap().split_whitespace().map(|word| word.parse::<i32>().unwrap()).collect();
        left.push(terms[0]);
        right.push(terms[1]);
    }
    left.sort_unstable();
    right.sort_unstable();
    (left,right)
    
}
fn calculate_distance(left:&Vec<i32>,right:&Vec<i32>)->i32{
    let res: i32 = left
    .into_iter()
    .zip(right.into_iter())
    .map(|(first, second)| (second - first).abs())
    .sum();
    res
}
fn calculate_similarity(left:&Vec<i32>,right:&Vec<i32>)->i32{
    let frequencies = right
          .iter()
          .copied()
          .fold(HashMap::new(), |mut map, val|{
              map.entry(val)
                 .and_modify(|frq|*frq+=1)
                 .or_insert(1);
              map
          });
    left.into_iter().map(|value| {
        value * frequencies.get(&value).cloned().unwrap_or(0)
    }).sum()
}