use std::cell::Cell;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
    
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_dir_from_raw(raw_dir: &str) -> (i32, i32) {
    match raw_dir {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => panic!()
    }
}

fn get_new_location(
    next_rope_pos: (i32, i32), 
    cur_rope_pos: (i32, i32)) -> (i32, i32) {
    let mut cur_rope_new_pos = cur_rope_pos;
    if (next_rope_pos.0 - cur_rope_pos.0).abs() <= 1 
        && (next_rope_pos.1 - cur_rope_pos.1).abs() <= 1 {
        // do nothing;
    } else if next_rope_pos.0 == cur_rope_pos.0 
        || next_rope_pos.1 == cur_rope_pos.1 {
        cur_rope_new_pos.0 = (next_rope_pos.0 + cur_rope_pos.0) / 2;
        cur_rope_new_pos.1 = (next_rope_pos.1 + cur_rope_pos.1) / 2;
    } else if (next_rope_pos.0 - cur_rope_pos.0).abs() == 1 {
        cur_rope_new_pos.0 = next_rope_pos.0;
        cur_rope_new_pos.1 = (next_rope_pos.1 + cur_rope_pos.1) / 2;
    } else if (next_rope_pos.1 - cur_rope_pos.1).abs() == 1 {
        cur_rope_new_pos.0 = (next_rope_pos.0 + cur_rope_pos.0) / 2;
        cur_rope_new_pos.1 = next_rope_pos.1;
    } else {
        cur_rope_new_pos.0 = (next_rope_pos.0 + cur_rope_pos.0) / 2;
        cur_rope_new_pos.1 = (next_rope_pos.1 + cur_rope_pos.1) / 2;
    }
    cur_rope_new_pos
}

fn main() {
    let mut visited_locations: HashSet<(i32, i32)> = HashSet::new();
    let mut ropes_locations: Vec<(i32, i32)> = vec![(0, 0); 10];
    visited_locations.insert(ropes_locations[9]);

    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let tokens: Vec<&str> = ip.split_whitespace().collect();
                let dir = get_dir_from_raw(tokens[0]);
                let steps = 
                    tokens[1].parse::<i32>().unwrap();

                for _ in 0..steps {
                    ropes_locations[0].0 += dir.0;
                    ropes_locations[0].1 += dir.1;

                    for idx in 1..10 {
                        ropes_locations[idx] = get_new_location(
                            ropes_locations[idx - 1], 
                            ropes_locations[idx]);
                    }

                    visited_locations.insert(ropes_locations[9]);
                }
            }
        }
    }

    println!("{}", visited_locations.len());
}