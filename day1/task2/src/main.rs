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
    let mut accumulate: i64 = 0;
    let mut sums: Vec<i64> = Vec::new();
    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if ip.trim().is_empty() {
                    sums.push(accumulate);
                    accumulate = 0;
                } else {
                    accumulate += ip.parse::<i64>().unwrap();
                }
            }
        }
    }
    sums.push(accumulate);
    sums.sort_by_key(|x| -x);
    println!("{}", sums[..3].iter().sum::<i64>());
}
