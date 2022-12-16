use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Vertex {
    edges: Vec<usize>,
    cost: i32,
}

fn generate_default_vertex() -> Vertex {
    Vertex { edges: Vec::new(), cost: 0 }
}

fn read_input() -> (Vec<Vertex>, usize) {
    let mut str_to_idx: HashMap<String, usize> = HashMap::new();
    let mut graph: Vec<Vertex> = Vec::new();
    let mut start_idx: Option<usize> = None;

    let regex = 
        Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d*); tunnels? leads? to valves? (.*)")
        .unwrap();

    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let tokens = regex.captures(&ip).unwrap();

                if !str_to_idx.contains_key(&tokens[1]) {
                    if &tokens[1] == "AA" {
                        start_idx = Some(str_to_idx.len());
                    }
                    str_to_idx.insert(
                        tokens[1].to_string(), str_to_idx.len());
                }

                let cur_v_idx = str_to_idx[&tokens[1]];
                if graph.len() <= cur_v_idx {
                    graph.resize_with(
                        cur_v_idx + 1, || generate_default_vertex());
                }
                
                graph[cur_v_idx].cost = tokens[2].parse::<i32>().unwrap();

                for token in tokens[3].split(", ") {
                    if !str_to_idx.contains_key(token) {
                        if token == "AA" {
                            start_idx = Some(str_to_idx.len());
                        }
                        str_to_idx.insert(
                            token.to_string(), str_to_idx.len());
                    }
                    graph[cur_v_idx].edges.push(str_to_idx[token]);
                }
            }
        }
    }

    (graph, start_idx.unwrap())
}

fn try_to_update_answer(old_value: &mut Option<i32>, new_value: i32) {
    if *old_value == None || old_value.unwrap() < new_value {
        *old_value = Some(new_value);
    }
}

fn calc_current_score(
    bitmask: usize, 
    cmp_index: &HashMap<usize, usize>, 
    graph: &Vec<Vertex>) -> i32 {
    let mut answer = 0;
    for (k, v) in cmp_index {
        if bitmask & (1 << v) != 0 {
            answer += graph[*k].cost;
        }
    }
    answer
}

fn main() {
    let (graph, start_idx) = read_input();
    let mut cmp_index: HashMap<usize, usize> = HashMap::new();

    for (idx, vertex) in graph.iter().enumerate() {
        if vertex.cost != 0 {
            cmp_index.insert(idx, cmp_index.len());
        }
    }

    let bitmask_cnt = 2_usize.pow(cmp_index.len() as u32);
    let mut dp: Vec<Vec<Vec<Vec<Option<i32>>>>> = 
        vec![vec![vec![vec![None; bitmask_cnt]; graph.len()]; graph.len()]; 27];
    dp[0][start_idx][start_idx][0] = Some(0);

    let mut bitmask_to_res: Vec<i32> = Vec::new();
    bitmask_to_res.reserve(bitmask_cnt);
    for bitmask in 0..bitmask_cnt {
        bitmask_to_res.push(calc_current_score(bitmask, &cmp_index, &graph));
    }

    for time in 0..26 {
        println!("time is {}", time);
        for vertex in 0..graph.len() {
            for vertex_el in 0..graph.len() {
                for bitmask in 0..bitmask_cnt {
                    if dp[time][vertex][vertex_el][bitmask] == None {
                        continue;
                    }

                    // Hacky hacky
                    let mod_ = cmp_index.contains_key(&vertex) 
                        && bitmask & (1 << cmp_index[&vertex]) == 0;
                    let mod_el = cmp_index.contains_key(&vertex_el) 
                        && bitmask & (1 << cmp_index[&vertex_el]) == 0
                        && vertex != vertex_el;

                    let new_score = 
                        dp[time][vertex][vertex_el][bitmask].unwrap() 
                        + bitmask_to_res[bitmask];

                    for neighbor_idx in 0..graph[vertex].edges.len() + mod_ as usize {
                        for neighbor_el_idx in 0..graph[vertex_el].edges.len() + mod_el as usize {
                            let mut new_bitmask = bitmask;
                            let mut neighbor = vertex;
                            let mut neighbor_el = vertex_el;

                            // Hacky hacky
                            if neighbor_idx == graph[vertex].edges.len() {
                                new_bitmask |= 1 << cmp_index[&vertex];
                            } 
                            if neighbor_el_idx == graph[vertex_el].edges.len() {
                                new_bitmask |= 1 << cmp_index[&vertex_el];
                            }

                            if neighbor_idx < graph[vertex].edges.len() {
                                neighbor = graph[vertex].edges[neighbor_idx];
                            }
                            if neighbor_el_idx < graph[vertex_el].edges.len() {
                                neighbor_el = graph[vertex_el].edges[neighbor_el_idx];
                            }

                            try_to_update_answer(
                                &mut dp[time + 1][neighbor][neighbor_el][new_bitmask], 
                                new_score);
                        }
                    } 
                }
            }
        }
    }

    let mut answer = None;
    for vertex in 0..graph.len() {
        for vertex_el in 0..graph.len() {
            for bitmask in 0..bitmask_cnt {
                if dp[26][vertex][vertex_el][bitmask] == None {
                    continue;
                }
                try_to_update_answer(
                    &mut answer, 
                    dp[26][vertex][vertex_el][bitmask].unwrap());
            }
        }
    }

    println!("{}", answer.unwrap());
}