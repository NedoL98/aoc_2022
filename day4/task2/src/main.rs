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

fn intervals_intersect(input: String) -> bool {
    let intervals_str: Vec<&str> = input.split(',').collect();
    assert!(intervals_str.len() == 2);

    let mut intervals: Vec<(i32, i32)> = Vec::new();
    intervals.reserve(intervals_str.len());

    for interval_str in intervals_str {
        let range: Vec<&str> = interval_str.split('-').collect();
        assert!(range.len() == 2);

        intervals.push((
            range[0].parse::<i32>().unwrap(), 
            range[1].parse::<i32>().unwrap()));
        assert!(intervals.last().unwrap().0 <= intervals.last().unwrap().1);
    }

    if intervals[0].0 > intervals[1].0 {
        intervals.swap(0, 1);
    }
    assert!(intervals[0].0 <= intervals[1].0);

    return intervals[0].1 >= intervals[1].0;
}

fn main() {
    let mut answer: i64 = 0;
    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                answer += intervals_intersect(ip) as i64;
            }
        }
    }
    println!("{}", answer);
}
