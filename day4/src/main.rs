use std::collections::HashSet;
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 0, "Need to specify input file");
    let filename = &args[1];
    let data = parse_file(&filename);
    // println!("{:?}",data)
    let _result1 = part1(&data);
    // println!("part1:{}", result1);
    let result2 = part2(&data);
    println!("part2:{}", result2);
}
fn parse_file(filename: &String) -> Vec<Vec<char>> {
    let data: Vec<Vec<char>> = fs::read_to_string(&filename)
        .expect("Unable to read file")
        .split('\n')
        .map(|x| x.chars().collect())
        .collect();
    data.clone()
}
fn part1(data: &Vec<Vec<char>>) -> usize {
    let search_term = "XMAS".to_string();
    let directions: [(i32, i32); 8] = [
        (0, 1),   // E
        (1, 0),   // S
        (-1, 0),  // N
        (0, -1),  // W
        (-1, 1),  // NE
        (-1, -1), // NW
        (1, 1),   // SE
        (1, -1),  // SW
    ];
    let num_rows = data.len();
    let num_cols = data[0].len();
    let mut count = 0;
    let mut start_points: Vec<(usize, usize)> = vec![];
    for i in 0..num_rows {
        for j in 0..num_cols {
            if data[i][j] == search_term.chars().nth(0).unwrap() {
                start_points.push((i, j));
            }
        }
    }

    start_points.into_iter().for_each(|(i, j)| {
        for direction in directions.iter() {
            let mut new_i = i as i32;
            let mut new_j = j as i32;
            let mut cursor = 0;

            while new_i >= 0 && new_i < num_rows as i32 && new_j >= 0 && new_j < num_cols as i32 {
                let char_at = search_term.chars().nth(cursor).unwrap();
                if data[new_i as usize][new_j as usize] == char_at {
                    cursor += 1;
                    if cursor == search_term.len() {
                        count += 1;
                        break;
                    }
                    new_i += direction.0;
                    new_j += direction.1;
                } else {
                    break;
                }
            }
        }
    });
    count
}
fn part2(data: &Vec<Vec<char>>) -> i32 {
    let search_term:HashSet<char> = HashSet::from_iter("MAS".to_string().chars());
    let directions: [[(i32, i32);2]; 2] = [
        [(-1, 1),  (1, -1)],  // NE+SW
        [(-1, -1), (1, 1)]   // NW+SE
        
        
    ];
    let num_rows = data.len();
    let num_cols = data[0].len();
    let mut count = 0;
    let mut start_points: Vec<(usize, usize)> = vec![];
    for i in 0..num_rows {
        for j in 0..num_cols {
            if data[i][j] == 'A' {
                start_points.push((i, j));
            }
        }
    }

    start_points.into_iter().for_each(|(i, j)| {
        let mut count_arms = 0;
        for direction in directions.iter() {
            let mut check_word:HashSet<char> = HashSet::new();  
            check_word.insert('A');
            for (dy,dx) in direction.iter(){
                let (new_i,new_j) =(i as i32 + dy,j as i32 + dx);
                if new_i >= 0 && new_i < num_rows as i32 && new_j >= 0 && new_j < num_cols as i32{
                    check_word.insert(data[new_i as usize][new_j as usize]);
                }
                else{
                    break;
                }
                    
            }
            if check_word == search_term{
                count_arms+=1;
            }
            
        }
        if count_arms ==2{
            count+=1;
        }
    });
    count
}
