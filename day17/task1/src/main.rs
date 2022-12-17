use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::max;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn char_to_dpos(ch: char) -> (i32, i32) {
    match ch {
        '<' => (0, -1),
        '>' => (0, 1),
        _ => panic!("unknown movement"),
    }
}

fn read_input() -> Vec<(i32, i32)> {
    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                return ip.chars().map(|ch| char_to_dpos(ch)).collect();
            }
        }
    }
    panic!("input is corrupted");
}

fn apply_movement(
    item: &Vec<(usize, usize)>, 
    item_pos: &mut (usize, usize),
    dpos: (i32, i32),
    grid: &Vec<Vec<bool>>) -> bool {

    for item_part in item {
        let new_item_part_pos = (
            (item_part.0 + item_pos.0) as i32 + dpos.0,
            (item_part.1 + item_pos.1) as i32 + dpos.1);
        
        if new_item_part_pos.0 < 0 {
            // Hit the floor
            return false;
        }

        if new_item_part_pos.1 < 0 || new_item_part_pos.1 >= 7 {
            // Hit the wall
            return false;
        }

        if !grid[new_item_part_pos.0 as usize][new_item_part_pos.1 as usize] {
            // Hit other blocks
            return false;
        }
    }

    // Apply movement
    *item_pos = (
        (item_pos.0 as i32 + dpos.0) as usize, 
        (item_pos.1 as i32 + dpos.1) as usize);

    return true;
}

fn main() {
    let input = read_input();
    let mut grid: Vec<Vec<bool>> = vec![vec![true; 7]; 10000];
    let mut grid_h = 0;
    let mut gas_idx = 0;

    let items: Vec<Vec<(usize, usize)>> = Vec::from([
        Vec::from([(0, 0), (0, 1), (0, 2), (0, 3)]),
        Vec::from([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]),
        Vec::from([(0, 0), (0, 1), (0, 2), (2, 2), (1, 2)]),
        Vec::from([(0, 0), (1, 0), (2, 0), (3, 0)]),
        Vec::from([(0, 0), (0, 1), (1, 0), (1, 1)])
    ]);

    for idx in 0..2022 {
    // for idx in 0..10 {
        let item = &items[idx % items.len()];
        let item_h = 
            item
            .iter()
            .map(|pos| pos.0)
            .max()
            .unwrap();
    
        let mut item_pos = (grid_h + 3, 2);
        // println!("{} {}", item_pos.0, item_pos.1);
        
        loop {
            let dpos = input[gas_idx];
            gas_idx = (gas_idx + 1) % input.len();

            // Move sideways
            apply_movement(&item, &mut item_pos, dpos, &grid);
        
            // Move down
            if !apply_movement(&item, &mut item_pos, (-1, 0), &grid) {
                // Stop

                for item_part in item {
                    grid[item_part.0 + item_pos.0][item_part.1 + item_pos.1] 
                        = false;
                }

                grid_h = max(grid_h, item_pos.0 + item_h + 1);
                break;
            }
        }

        // for x in (0..10).rev() {
        //     for y in 0..7 {
        //         if grid[x][y] {
        //             print!(".");
        //         } else {
        //             print!("#");
        //         }
        //     }
        //     println!("");
        // }
        // println!("");
// 
        // println!("{}", grid_h);
    }

    println!("{}", grid_h);
}