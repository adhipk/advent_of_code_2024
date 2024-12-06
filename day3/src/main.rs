
// use regex::Regex;
use regex::Regex;
use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let data = parse_file(filename);
    // println!("{data}");
    let result = find_muls(data);
    println!("part1:{:?}",result);
}
fn parse_file(filename: &String) -> String {
    let contents = fs::read_to_string(filename).expect("Unable to read file");
    contents
}
fn find_muls(contents:String) -> i32{
    let re = Regex::new(r"(do\(\))|(don't\(\))|(mul\((\d+)\,(\d+)\))").unwrap();
    
    let mut toggle = true;
    
    let mut result = 0;
    
    re.captures_iter(&contents).for_each(|c|{
        let mut groups:Vec<&str> = vec![];
        for i in 1..6{
            let group = c.get(i);
            if group ==None{
                groups.push("");
            }else{
                groups.push(group.unwrap().as_str());
            }
        }
        if groups[0].len() >0{
            toggle = true;
        }else if groups[1].len() >0{
            toggle = false;
        }
        if groups[2].len() >0 && toggle{
            let (val1,val2) = (groups[3].parse::<i32>().unwrap(),groups[4].parse::<i32>().unwrap());
            result +=val1 * val2    
        }
    });
    result
}
