
use std::env;
use std::fs;
mod tree;
use tree::Node;

fn can_reach_target_with_concat(numbers:Vec<i64>,target:i64)->bool{
    let mut tree = Node::new(numbers[0]);
    
    tree.build_tree(numbers[1..].to_vec(),target);
    let result = tree.search(target);
    result
}
fn can_reach_target(numbers:Vec<i64>,target:i64)->bool{
    let mut tree = Node::new(numbers[0]);
    
    tree.build_tree(numbers[1..].to_vec());
    let result = tree.search_lr(target);
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 0, "Need to specify input file");
    let filename = &args[1];
    let input_text = parse_file(&filename);
    let result1 = part1(&input_text);
    println!("part1:{}", result1);
    let result2 = part2(&input_text);
    println!("part2:{}", result2);
}

fn parse_file(filename: &String) -> String {
    let contents = fs::read_to_string(&filename).expect("Unable to read file");
    contents.clone()
}


fn part1(input_text:&String)->i64{
    let mut result = 0;
    for line in input_text.lines(){
        let parts:Vec<String> = line.split(':').map(str::to_string).collect::<Vec<String>>();
        let target = &parts[0];
        let numbers = &parts[1];
        let target = target.parse::<i64>().unwrap();
        let numbers = numbers.split_whitespace().into_iter().map(|number|{
            number.parse::<i64>().unwrap()
        }).collect::<Vec<i64>>();
        if can_reach_target(numbers, target){
            result+=target;
        }
    }
    result
    
    
}
fn part2(input_text:&String)->i64{
    let mut result = 0;
    for line in input_text.lines(){
        let parts:Vec<String> = line.split(':').map(str::to_string).collect::<Vec<String>>();
        let target = &parts[0];
        let numbers = &parts[1];
        let target = target.parse::<i64>().unwrap();
        let numbers = numbers.split_whitespace().into_iter().map(|number|{
            number.parse::<i64>().unwrap()
        }).collect::<Vec<i64>>();
        if can_reach_target_with_concat(numbers, target){
            result+=target;
        }
    }
    result
    
} 