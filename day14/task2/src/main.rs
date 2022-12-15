use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::{min, max};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn add_obstacle_to_grid(grid: &mut Vec<Vec<bool>>, obstacles_str: &String) {
    let obstacles: Vec<(usize, usize)> = 
        obstacles_str
        .split(" -> ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|token| {
            let pair: Vec<&str> = token.split(',').collect();
            assert!(pair.len() == 2);
            (pair[0].parse::<usize>().unwrap(), 
            pair[1].parse::<usize>().unwrap())
        })
        .collect();

    for idx in 0..obstacles.len() - 1 {
        if obstacles[idx].0 != obstacles[idx + 1].0 {
            let startx = min(obstacles[idx].0, obstacles[idx + 1].0);
            let finishx = max(obstacles[idx].0, obstacles[idx + 1].0);
            for dx in startx..finishx + 1 {
                grid[dx][obstacles[idx].1] = false;
            }
        } else {
            let starty = min(obstacles[idx].1, obstacles[idx + 1].1);
            let finishy = max(obstacles[idx].1, obstacles[idx + 1].1);
            for dy in starty..finishy + 1 {
                grid[obstacles[idx].0][dy] = false;
            }
        }
    }
}

fn read_input() -> Vec<Vec<bool>> {
    let mut grid: Vec<Vec<bool>> = vec![vec![true; 1000]; 1000];
    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                add_obstacle_to_grid(&mut grid, &ip);
            }
        }
    }

    let mut max_y = 0;
    for j in 0..1000 {
        for i in 0..1000 {
            if !grid[i][j] {
                max_y = j;
            }
        }
    }

    println!("max y coord is {}", max_y);
    assert!(max_y + 2 < grid[0].len());
    for i in 0..1000 {
        grid[i][max_y + 2] = false;
    }

    grid
}

fn add_stone(grid: &mut Vec<Vec<bool>>) -> bool {
    let mut pos: (usize, usize) = (500, 0);
    if !grid[pos.0][pos.1] {
        return false;
    }
    
    loop {
        assert!(pos.1 + 1 < grid[0].len(), "wtf");
        assert!(pos.0 > 0 && pos.0 + 1 < grid.len(), "grid is too small!");
        if grid[pos.0][pos.1 + 1] {
            pos.1 += 1;
        } else if grid[pos.0 - 1][pos.1 + 1] {
            pos.0 -= 1;
            pos.1 += 1;
        } else if grid[pos.0 + 1][pos.1 + 1] {
            pos.0 += 1;
            pos.1 += 1;
        } else {
            grid[pos.0][pos.1] = false;
            break;
        }
    }

    true
}

fn main() {
    let mut grid = read_input();
    let mut answer = 0;

    println!("reading ok!");
    loop {
        if !add_stone(&mut grid) {
            break;
        }
        answer += 1;
    }

    println!("{}", answer);
}