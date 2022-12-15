use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Sensor {
    pos: (i32, i32),
    dist: i32,
}

fn get_dist(lhs: (i32, i32), rhs: (i32, i32)) -> i32 {
    (lhs.0 - rhs.0).abs() + (lhs.1 - rhs.1).abs()
}

fn read_input() -> (Vec<Sensor>, Vec<(i32, i32)>) {
    let mut sensors: Vec<Sensor> = Vec::new();
    let mut beacons: Vec<(i32, i32)> = Vec::new();

    let regex = 
        Regex::new(r"Sensor at x=(-?\d*), y=(-?\d*): closest beacon is at x=(-?\d*), y=(-?\d*)")
        .unwrap();

    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let tokens = regex.captures(&ip).unwrap();
                let sensor_pos = (
                    tokens[1].parse::<i32>().unwrap(), 
                    tokens[2].parse::<i32>().unwrap());
                let beacon_pos = (
                    tokens[3].parse::<i32>().unwrap(), 
                    tokens[4].parse::<i32>().unwrap());
                let dist = get_dist(sensor_pos, beacon_pos);
                sensors.push(Sensor{ pos: sensor_pos, dist: dist });
                beacons.push(beacon_pos);
            }
        }
    }

    (sensors, beacons)
}

fn main() {
    let (sensors, beacons) = read_input();

    let mut answer = 0;
    let y = 2000000;
    for x in (-10_i32.pow(7))..(10_i32.pow(7)) {
        let cur_pos = (x, y);
        if beacons.contains(&cur_pos) {
            continue;
        }
        
        for sensor in &sensors {
            if get_dist(sensor.pos, cur_pos) <= sensor.dist {
                answer += 1;
                break;
            }
        }
    }
    println!("{}", answer);
}