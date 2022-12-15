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

fn add_score(prev_score: &mut i64, l_token: &str, r_token: &str) {
    assert!(l_token.len() == 1);
    assert!(r_token.len() == 1);
    let mut l_int = (l_token.as_bytes()[0] - ('A' as u8)) as i64;
    let r_int = (r_token.as_bytes()[0] - ('X' as u8)) as i64;

    let your_num = (l_int + (r_int - 1) + 3) % 3 + 1;

    l_int += 1;

    *prev_score += your_num + ((your_num - l_int + 4) % 3) * 3;
}

fn main() {
    let mut total: i64 = 0;
    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let tokens: Vec<&str> = ip.split_whitespace().collect();
                add_score(&mut total, tokens[0], tokens[1]);
            }   
        }
    }
    print!("{}\n", total);
}
