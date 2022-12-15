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

fn try_to_update_field(
    field: &mut Vec<bool>, sprite_pos: i32, cur_ts: i32) {
    if cur_ts < field.len() as i32 {
        field[cur_ts as usize] = (sprite_pos - cur_ts % 40).abs() <= 1;
    }
}

fn main() {
    let mut field: Vec<bool> = vec![false; 240];

    let mut sprite_pos = 1;
    let mut cur_ts: i32 = 0;

    try_to_update_field(&mut field, sprite_pos, cur_ts);

    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let tokens: Vec<&str> = ip.split_whitespace().collect();
                match tokens[0] {
                    "noop" => {
                        cur_ts += 1;
                    },
                    "addx" => {
                        cur_ts += 1;
                        try_to_update_field(&mut field, sprite_pos, cur_ts);
                        cur_ts += 1;
                        sprite_pos += tokens[1].parse::<i32>().unwrap();
                    },
                    _ => panic!("unknown operation")
                }
                try_to_update_field(&mut field, sprite_pos, cur_ts);
            }
        }
    }

    for row in 0..6 {
        for col in 0..40 {
            match field[row * 40 + col] {
                true => print!("#"),
                false => print!(".")
            }
        }
        print!("\n");
    }
}