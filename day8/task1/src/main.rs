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

fn main() {
    let mut input: Vec<Vec<u8>> = Vec::new();

    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                input.push(Vec::new());
                let last_idx = input.len() - 1;
                for char in ip.trim().as_bytes() {
                    let num = char - ('0' as u8);
                    assert!(num <= 9);
                    input[last_idx].push(num);
                }
            }
        }
    }

    let mut is_visible: Vec<Vec<bool>> = 
        vec![vec![false; input[0].len()]; input.len()];

    for (i, row) in input.iter().enumerate() {
        let mut max_height: i8 = -1;
        for j in 0..row.len() {
            if row[j] as i8 > max_height {
                is_visible[i][j] = true;
                max_height = row[j].try_into().unwrap();
            }
        }
        max_height = -1;
        for j in (0..row.len()).rev() {
            if row[j] as i8 > max_height {
                is_visible[i][j] = true;
                max_height = row[j].try_into().unwrap();
            }
        }
    }

    for j in 0..input[0].len() {
        let mut max_height: i8 = -1;
        for i in 0..input.len() {
            if input[i][j] as i8 > max_height {
                is_visible[i][j] = true;
                max_height = input[i][j].try_into().unwrap();
            }
        }
        max_height = -1;
        for i in (0..input.len()).rev() {
            if input[i][j] as i8 > max_height {
                is_visible[i][j] = true;
                max_height = input[i][j].try_into().unwrap();
            }
        }
    }

    let mut answer = 0;
    for row in is_visible {
        for cell in row {
            answer += cell as i32;
        }
    }

    println!("{}", answer);
}