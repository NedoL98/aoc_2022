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

fn calc_size(answer: &mut i64, lines: &Vec<String>, line_idx: &mut usize) -> i64 {
    let mut cur_size: i64 = 0;
    while *line_idx < lines.len() && lines[*line_idx] != "$ cd .." {
        let cur_line = &lines[*line_idx];

        if cur_line.starts_with("$ cd") {
            // Descent into children directory
            *line_idx += 1;
            cur_size += calc_size(answer, lines, line_idx);
        } else if cur_line != "$ ls" {
            // File or directory
            let tokens: Vec<&str> = cur_line.split_whitespace().collect();
            if tokens[0] != "dir" {
                // Definetely file
                cur_size += tokens[0].parse::<i64>().unwrap();
            }
        }

        *line_idx += 1;
    }

    if *answer > cur_size && cur_size >= 45349983 - 40000000 {
        *answer = cur_size;
    } 

    return cur_size;
}

fn main() {
    let mut answer: i64 = i64::MAX;
    let mut line_idx: usize = 0;

    let mut input: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                input.push(ip);
            }
        }
    }
    calc_size(&mut answer, &input, &mut line_idx);
    println!("{}", answer);
}