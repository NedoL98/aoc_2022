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
    let mut stacks: Vec<String> = vec![
        String::from("ZJNWPS"),
        String::from("GST"),
        String::from("VQRLH"),
        String::from("VSTD"),
        String::from("QZTDBMJ"),
        String::from("MWTJDCZL"),
        String::from("LPMWGTJ"),
        String::from("NGMTBFQH"),
        String::from("RDGCPBQW")
    ];

    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let tokens: Vec<&str> = ip.split_whitespace().collect();
                let cnt: usize = tokens[1].parse::<usize>().unwrap();
                let from: usize = tokens[3].parse::<usize>().unwrap() - 1;
                let to: usize = tokens[5].parse::<usize>().unwrap() - 1;
                
                let start_pos = stacks[from].len() - cnt;
                let suffix = stacks[from].split_off(start_pos);
                stacks[to] += &suffix;
            }
        }
    }
    for stack in &stacks {
        print!("{}", stack.as_bytes()[stack.len() - 1] as char);
    }
    println!("");
}
