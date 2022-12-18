use std::fs::File;
use std::hash::Hash;
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

#[derive(Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}

fn read_input() -> HashSet<Position> {
    let mut positions: HashSet<Position> = HashSet::new();
    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let tokens: Vec<&str> = ip.split(',').collect();
                // Add one so that the minimum value for each coordinate is 1
                positions.insert(Position {
                    x: tokens[0].parse::<usize>().unwrap(),
                    y: tokens[1].parse::<usize>().unwrap(),
                    z: tokens[2].parse::<usize>().unwrap() + 1,
                });
            }
        }
    }

    positions
}

fn is_in_cube(x: i32, y: i32, z: i32, cube_size: i32) -> bool {
    return x >= 0 && x < cube_size
        && y >= 0 && y < cube_size
        && z >= 0 && z < cube_size;
}

fn dfs(x: usize, 
        y: usize, 
        z: usize, 
        used: &mut Vec<Vec<Vec<bool>>>, 
        cubes_touched: &mut i32,
        positions: &HashSet<Position>) {
    used[x][y][z] = true;
    for dx in -1_i32..1 + 1 {
        for dy in -1_i32..1 + 1 {
            for dz in -1_i32..1 + 1 {
                if dx.abs() + dy.abs() + dz.abs() != 1 {
                    continue;
                }

                if !is_in_cube(
                    x as i32 + dx, 
                    y as i32 + dy, 
                    z as i32 + dz,
                    used.len() as i32) {
                    continue;
                }
                
                let new_x = (x as i32 + dx) as usize;
                let new_y = (y as i32 + dy) as usize;
                let new_z = (z as i32 + dz) as usize;
                if positions.contains(&Position{x: new_x, y: new_y, z: new_z}) {
                    *cubes_touched += 1;
                    continue;
                }

                if !used[new_x][new_y][new_z] {
                    dfs(new_x, new_y, new_z, used, cubes_touched, positions);
                }
            }
        }
    }
}

fn main() {
    let positions = read_input();
    const CUBE_SIZE: usize = 25;
    let mut used: Vec<Vec<Vec<bool>>> 
        = vec![vec![vec![false; CUBE_SIZE]; CUBE_SIZE]; CUBE_SIZE];

    let mut cubes_touched = 0;
    dfs(0, 0, 0, &mut used, &mut cubes_touched, &positions);

    println!("{}", cubes_touched);
}