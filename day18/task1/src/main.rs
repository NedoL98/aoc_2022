use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Position {
    x: i32,
    y: i32,
    z: i32,
}

fn read_input() -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let tokens: Vec<&str> = ip.split(',').collect();
                positions.push(Position {
                    x: tokens[0].parse::<i32>().unwrap(),
                    y: tokens[1].parse::<i32>().unwrap(),
                    z: tokens[2].parse::<i32>().unwrap(),
                });
            }
        }
    }
    positions
}

fn positions_are_adjacent(lhs: &Position, rhs: &Position) -> bool {
    return 
        (lhs.x - rhs.x).abs() 
        + (lhs.y - rhs.y).abs() 
        + (lhs.z - rhs.z).abs() == 1; 
}

fn main() {
    let positions = read_input();
    let mut answer = positions.len() * 6;
    for i in 0..positions.len() {
        for j in 0..positions.len() {
            if positions_are_adjacent(&positions[i], &positions[j]) {
                answer -= 1;
            }
        }
    }

    println!("{}", answer);
}