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

fn read_input() -> Vec<(i64, usize)> {
    let mut numbers: Vec<(i64, usize)> = Vec::new();
    let mut idx = 0;

    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                numbers.push((ip.parse::<i64>().unwrap() * 811589153, idx));
                idx += 1;
            }
        }
    }
    numbers
}

fn main() {
    let mut numbers = read_input();
    let numbers_cnt = numbers.len();

    for outer_it in 0..10 {
        for it in 0..numbers_cnt as usize {
            let mut pos = 0;
            for (idx, (_, initial_idx)) in numbers.iter().enumerate() {
                if *initial_idx == it {
                    pos = idx;
                }
            }

            let circle_size = (numbers_cnt - 1) as i64;
            let mut delta_refined = 
                ((numbers[pos].0 % circle_size) + circle_size) % circle_size;
            assert!(
                delta_refined >= 0 && delta_refined < circle_size, 
                "invalid refined value");

            while delta_refined > 0 {
                numbers.swap(pos, (pos + 1) % numbers_cnt);
                pos = (pos + 1) % numbers_cnt;
                delta_refined -= 1;
            }
        }
        println!("{} outer loop done", outer_it);
    }

    let mut pos = 0;
    for (idx, (number, _)) in numbers.iter().enumerate() {
        if *number == 0 {
            pos = idx;
            break;
        }
    }

    let mut answer = 0;
    let mut it = 0;
    while it <= 3000 {
        it += 1;
        pos = (pos + 1) % numbers_cnt;
        if it % 1000 == 0 {
            answer += numbers[pos].0;
        }
    }

    println!("{}", answer);
}
