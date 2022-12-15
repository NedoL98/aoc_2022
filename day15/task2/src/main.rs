use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::cmp::max;

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
    let (sensors, _beacons) = read_input();

    for x in 0..4000001 {
        if x % 100000 == 0 {
            println!("x is {}", x);
        }

        let mut intervals: Vec<(i32, i32)> = Vec::new();
        for sensor in &sensors {
            if (sensor.pos.0 - x).abs() > sensor.dist {
                continue;
            }
            let dist_diff = sensor.dist - (sensor.pos.0 - x).abs();
            intervals.push((sensor.pos.1 - dist_diff, sensor.pos.1 + dist_diff));
        }
        
        intervals.sort();
        assert!(intervals[0].0 <= 0);
        let mut max_r = intervals[0].1;
        for idx in 1..intervals.len() {
            if max_r + 1 < intervals[idx].0 {
                println!("{}", (x as i64) * 4000000 + ((max_r + 1) as i64));
            } 
            max_r = max(max_r, intervals[idx].1);
        }
    }
}