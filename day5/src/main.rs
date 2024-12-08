use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 0, "Need to specify input file");
    let filename = &args[1];
    let (rules, updates) = parse_file(&filename);
    let result1 = part1(rules.clone(), &updates);
    println!("part1:{}", result1);
    let result2 = part2(rules.clone(), &updates);
    println!("part2:{}", result2);
}

fn parse_file(filename: &String) -> (Vec<(i32,i32)>, Vec<Vec<i32>>) {
    let contents = fs::read_to_string(&filename).expect("Unable to read file");
    let mut rules: Vec<(i32,i32)> = vec![];
    let mut updates: Vec<Vec<i32>> = vec![];
    let contents = contents
        .split("\n\n")
        .map(str::to_string)
        .collect::<Vec<String>>();
    // parse rules as vector of tuples
    for edge_line in contents[0].lines() {
        let edge = edge_line
            .split('|')
            .map(|node| node.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        rules.push((edge[0],edge[1]));
    }
    // parse the rest of the lines as updates in that graph
    for updates_line in contents[1].lines() {
        let update = updates_line
            .split(",")
            .map(|node| node.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        updates.push(update);
    }
    (rules.clone(), updates.clone())
}

fn part1(rules: Vec<(i32,i32)>, updates: &Vec<Vec<i32>>) -> i32 {
    
    updates.into_iter().filter(|update|{
        let (indegree,adj_list) = calculate_graph_params(rules.clone(),*update);
        let topological_order = get_topological_order(indegree,adj_list);
        &topological_order == *update
    }).map(|x| x[x.len()/2]).sum()
}

// calculate adj_list and in_degree based on filtered rules for the update
fn calculate_graph_params(rules:Vec<(i32,i32)>, update:&Vec<i32>)->(HashMap<i32,i32>, HashMap<i32,Vec<i32>>){
    let filtered_rules = rules.into_iter().filter(|rule|{
        update.contains(&rule.0) && update.contains(&rule.1)
    });
    
    let mut adj_list: HashMap<i32,Vec<i32>> = HashMap::new();
    filtered_rules.for_each(|(source,sink)|{
        adj_list.entry(source).or_insert(vec![]).push(sink);
    });
    let mut indegree: HashMap<i32, i32> = HashMap::new();
    for (node, neighbors) in adj_list.clone() {
        indegree.entry(node).or_insert(0);
        for neighbor in neighbors {
            *indegree.entry(neighbor).or_insert(0) += 1;
        }
    }
    (indegree,adj_list)
    
    
}


fn get_topological_order(mut indegree:HashMap<i32,i32>,adj_list:HashMap<i32,Vec<i32>>)->Vec<i32>{
    let mut queue:VecDeque<i32> = VecDeque::new();
    for (vertex,degree) in indegree.iter(){
        if *degree == 0{
            queue.push_back(*vertex);
        }
    }
    let mut result = vec![];
    while !queue.is_empty(){
        let node = queue.pop_front().unwrap();
        result.push(node);
        let adj_nodes = adj_list.get(&node);
        if !adj_nodes.is_none(){
            
            let adj_nodes = adj_nodes.unwrap();
            for adj_node in adj_nodes{
                *indegree.get_mut(adj_node).unwrap() -=1;
                if *indegree.get(adj_node).unwrap() == 0{
                    queue.push_back(*adj_node);
                }
            }
        }
    }
    result
}
fn part2(rules: Vec<(i32,i32)>, updates: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    updates.into_iter().for_each(|update|{
        let (indegree,adj_list) = calculate_graph_params(rules.clone(),update);
        let topological_order = get_topological_order(indegree,adj_list);
        if topological_order != *update{
           result +=topological_order[topological_order.len()/2]
        }
    });
    result
}
