use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Input {
    grid: Vec<Vec<i32>>,
    start: (usize, usize),
}

fn generate_empty_input() -> Input {
    Input {
        grid: Vec::new(),
        start: (0, 0),
    }
}

fn get_height(s: &str) -> i32 {
    if s == "S" {
        0
    } else if s == "E" {
        'z' as i32 - 'a' as i32
    } else {
        assert!(s.len() == 1, "Tokens is not a char");
        s.as_bytes()[0] as i32 - 'a' as i32
    }
}

fn parse_input() -> Input {
    let mut input = generate_empty_input();
    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let tokens: Vec<&str> = 
                    ip
                    .trim()
                    .split("")
                    .filter(|s| !s.is_empty())
                    .collect();
                if let Some(pos) = tokens.iter().position(|&s| s == "E") {
                    input.start = (input.grid.len(), pos);
                }

                let heights: Vec<i32> = 
                    tokens
                    .iter()
                    .map(|s| get_height(s))
                    .collect();
                input.grid.push(heights);
            }
        }
    }
    input
}

fn main() {
    let input  = parse_input(); 

    let mut distances: Vec<Vec<Option<i32>>> = 
        vec![vec![None; input.grid[0].len()]; input.grid.len()];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    distances[input.start.0][input.start.1] = Some(0);
    queue.push_front(input.start);

    let mut answer: Option<i32> = None;
    while !queue.is_empty() {
        let v = queue.pop_back().unwrap();
        
        if input.grid[v.0][v.1] == 0 {
            if answer == None || answer.unwrap() > distances[v.0][v.1].unwrap() {
                answer = Some(distances[v.0][v.1].unwrap());
            }
        }

        for dx in vec![-1, 0, 1] {
            for dy in vec![-1, 0, 1] {
                if (dx != 0) ^ (dy != 0) == false {
                    continue;
                }
                if v.0 as i32 + dx >= 0 
                    && v.0 as i32 + dx < distances.len() as i32
                    && v.1 as i32 + dy >= 0 
                    && v.1 as i32 + dy < distances[0].len() as i32 {
                    let new_pos: (usize, usize) = (
                        (v.0 as i32 + dx) as usize, (v.1 as i32 + dy) as usize);
                    if input.grid[new_pos.0][new_pos.1] + 1 >= input.grid[v.0][v.1]  
                        && distances[new_pos.0][new_pos.1] == None {
                        distances[new_pos.0][new_pos.1] = 
                            Some(distances[v.0][v.1].unwrap() + 1);
                        queue.push_front(new_pos);
                    }
                }
            } 
        }
    }
    println!("{}", answer.unwrap());
}