use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::iter::Peekable;    

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn try_to_move_iter<'a, I>(
    next_timepoint_it: &mut Peekable<I>, 
    cur_ts: i32, 
    answer: &mut i32,
    cur_value: i32)
where I: Iterator<Item = &'a i32> {
    if let Some(next_timepoint) = next_timepoint_it.peek() {
        if cur_ts >= **next_timepoint {
            *answer += cur_value * (**next_timepoint);
            next_timepoint_it.next();
        }
    }
}

fn main() {
    let interesting_timepoints = vec![20, 60, 100, 140, 180, 220];
    let mut next_timepoint_it = interesting_timepoints.iter().peekable();

    let mut answer = 0;

    let mut cur_value = 1;
    let mut cur_ts = 1;

    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let tokens: Vec<&str> = ip.split_whitespace().collect();
                match tokens[0] {
                    "noop" => {
                        cur_ts += 1;
                    },
                    "addx" => {
                        try_to_move_iter(
                            &mut next_timepoint_it, 
                            cur_ts + 1, 
                            &mut answer, 
                            cur_value);
                        cur_ts += 2;
                        cur_value += tokens[1].parse::<i32>().unwrap();
                    },
                    _ => panic!("unknown operation")
                }

                try_to_move_iter(
                    &mut next_timepoint_it, 
                    cur_ts, 
                    &mut answer, 
                    cur_value);
            }
        }
    }

    println!("{}", answer);
}