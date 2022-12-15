use std::cell::Cell;
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

    let mut answer = 1;

    for i in 1..input.len() - 1 {
        for j in 1..input[0].len() - 1 {
            let mut cur_answer: i32 = 1;
            let cell_value = input[i][j];

            for di in i + 1..input.len() {
                if input[di][j] >= cell_value || di + 1 == input.len() {
                    cur_answer *= (di - i) as i32;
                    break;
                }
            }
            for di in (0..i - 1).rev() {
                if input[di][j] >= cell_value || di == 0 {
                    cur_answer *= (i - di) as i32;
                    break;
                }
            }

            for dj in j + 1..input[0].len() {
                if input[i][dj] >= cell_value || dj + 1 == input[0].len() {
                    cur_answer *= (dj - j) as i32;
                    break;
                }
            }
            for dj in (0..j - 1).rev() {
                if input[i][dj] >= cell_value || dj == 0 {
                    cur_answer *= (j - dj) as i32;
                    break;
                }
            }       

            if answer < cur_answer {
                answer = cur_answer;
            }
        }
    }

    println!("{}", answer);
}