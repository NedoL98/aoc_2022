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

fn main() {
    let mut visited_locations: HashSet<(i32, i32)> = HashSet::new();
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    visited_locations.insert(tail_pos);

    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let tokens: Vec<&str> = ip.split_whitespace().collect();
                let dir = get_dir_from_raw(tokens[0]);
                let steps = 
                    tokens[1].parse::<i32>().unwrap();

                for _ in 0..steps {
                    head_pos.0 += dir.0;
                    head_pos.1 += dir.1;

                    if (head_pos.0 - tail_pos.0).abs() <= 1 
                        && (head_pos.1 - tail_pos.1).abs() <= 1 {
                        // do nothing;
                    } else if head_pos.0 == tail_pos.0 
                        || head_pos.1 == tail_pos.1 {
                        tail_pos.0 = (head_pos.0 + tail_pos.0) / 2;
                        tail_pos.1 = (head_pos.1 + tail_pos.1) / 2;
                    } else if (head_pos.0 - tail_pos.0).abs() == 1 {
                        tail_pos.0 = head_pos.0;
                        tail_pos.1 = (head_pos.1 + tail_pos.1) / 2;
                    } else {
                        tail_pos.0 = (head_pos.0 + tail_pos.0) / 2;
                        tail_pos.1 = head_pos.1;
                    }

                    visited_locations.insert(tail_pos);
                }
            }
        }
    }

    println!("{}", visited_locations.len());
}