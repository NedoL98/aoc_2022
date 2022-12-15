use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
    
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calculate_answer(line: String) -> usize {
    let mut letter_set: HashMap<u8, i32> = HashMap::new();
    for (idx, letter) in line.as_bytes().iter().enumerate() {
        letter_set
            .entry(*letter)
            .and_modify(|count| *count += 1)
            .or_insert(1);        
        if idx > 13 {
            let prev_letter = 
                line.as_bytes()[(idx - 14) as usize];
            if letter_set[&prev_letter] == 1 {
                letter_set.remove(&prev_letter);
            } else {
                letter_set
                    .entry(prev_letter)
                    .and_modify(|count| *count -= 1);
            }
        } 
        if idx >= 13 && letter_set.len() == 14 {
            return idx + 1;
        }
    }
    panic!("this can't be right!");
}

fn main() {
    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", calculate_answer(ip));
            }
        }
    }
}
