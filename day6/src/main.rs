
use std::collections::HashSet;
use std::env;

use std::fs;

use std::{thread, time};

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 0, "Need to specify input file");
    let filename = &args[1];
    let data = parse_file(&filename);
    // debug_grid(&data);
    let result1 = part1(&data);
    println!("part1:{}", result1);
    let result2 = part2(&data);
    println!("part2:{}", result2);
}
fn draw_grid(grid:&Vec<Vec<char>>){
    print!("\x1B[2J\x1B[1;1H");
    for line in grid{
        let s: String = line.into_iter().collect();
        println!("{s}");
    }
    let hun_millis = time::Duration::from_millis(100);
    thread::sleep(hun_millis);
}
fn get_guard_position(grid:&Vec<Vec<char>>)->(usize,usize,usize){
    let mut guard:(usize,usize,usize) = (0,0,0);
    for i in 0..grid.len(){
        for j in 0..grid[0].len(){
            
            match grid[i][j]{
                '>'=> {
                    guard = (i, j, 0);
                }
                'v'=> { guard = (i, j, 1); }
                '<'=> {guard = (i, j, 2);}
                '^'=> {guard = (i, j, 3);}
                _ => {continue}
            }
        }
    }
    guard
}
fn traverse_grid(grid:&Vec<Vec<char>>,guard:(usize,usize,usize),animate_grid:bool)->Option<HashSet<(usize,usize,usize)>>{
    
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let directions: [(i32, i32); 4] = [
        (0, 1),   // E
        (1, 0),   // S
        (0, -1),  // W
        (-1, 0),  // N
    ];
    
    let mut result:HashSet<(usize,usize,usize)> = HashSet::new();
    // insert guard starting position
    result.insert(guard);
    let (mut i,mut j,mut dir_index) = guard;
    let mut debug_grid = grid.clone();
    while in_range(i,j,num_rows,num_cols) {
        let dir = directions[dir_index];
        let new_i = ((i as i32) + dir.0) as usize;
        let new_j = ((j as i32) + dir.1) as usize;
        if !in_range(new_i, new_j, num_rows, num_cols){
            break;
        }
        if grid[new_i][new_j] == '#'{
            // obstacle change direction
            dir_index = (dir_index+1)%4;
        }else{
            if animate_grid{
                debug_grid[i][j] = 'X';
                debug_grid[new_i][new_j] = match dir_index{
                    0=>{'>'}
                    1=>{'v'}
                    2=>{'<'}
                    3=>{'^'}
                    _=>'N'
                };
                draw_grid(&debug_grid);    
            }
            
            i = new_i;
            j = new_j;
        }
        if result.get(&(i,j,dir_index)).is_none(){
            
            result.insert((i,j,dir_index));    
        }else{
            // cycle detected
            return None
        }
        
        
        
    }
    Some(result)
        
}

fn in_range(i:usize,j:usize,num_rows:usize,num_cols:usize)->bool{
    (i) < num_rows && (j as usize) < num_cols
}
fn parse_file(filename: &String) -> Vec<Vec<char>> {
    let data: Vec<Vec<char>> = fs::read_to_string(&filename)
        .expect("Unable to read file")
        .split('\n')
        .map(|x| x.chars().collect())
        .collect();
    data.clone()
}
fn part1(grid:&Vec<Vec<char>>)->i32{
    let mut dist_pos:HashSet<(usize,usize)> = HashSet::new();
    let guard = get_guard_position(&grid);
    traverse_grid(grid,guard,false).unwrap().into_iter().for_each(|pos|{
        dist_pos.insert((pos.0,pos.1));
    });
    dist_pos.len() as i32
}
fn part2(grid:&Vec<Vec<char>>)->i32{
    let mut result = 0;
    let mut positions:HashSet<(usize,usize)> = HashSet::new();
    let guard = get_guard_position(&grid);
    traverse_grid(grid,guard,false).unwrap().into_iter().for_each(|(i,j,_)|{
        if (i,j) != (guard.0,guard.1){
            positions.insert((i,j));
        }
        
    });
    
    // remove guard init position
    for (i,j) in positions{
        let mut local_grid = grid.clone();
        local_grid[i][j] = '#';
        if traverse_grid(&local_grid,guard,false).is_none(){
            result+=1;
        }
    }
    
    result
}