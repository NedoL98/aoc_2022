use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[allow(non_upper_case_globals)]
fn get_priority(rucksacks: &Vec<String>) -> u8 {
    let mut rucksacks_items: Vec<HashSet<u8>> = Vec::new();

    for rucksack in rucksacks {
        rucksacks_items.push(rucksack.as_bytes().iter().copied().collect());
    }

    let intersection_tmp = rucksacks_items[0]
        .intersection(&rucksacks_items[1])
        .copied()
        .collect::<HashSet<u8>>();

    let intersection: HashSet<&u8> = 
        intersection_tmp
        .intersection(&rucksacks_items[2])
        .collect();
    assert!(intersection.len() == 1);
    let elem = **intersection.iter().next().unwrap();

    const a_u8: u8 = 'a' as u8;
    const z_u8: u8 = 'z' as u8;
    const A_u8: u8 = 'A' as u8;
    const Z_u8: u8 = 'Z' as u8;

    return match elem {
        a_u8..=z_u8 => elem as u8 - 'a' as u8 + 1,
        A_u8..=Z_u8 => elem as u8 - 'A' as u8 + 27,
        _ => panic!(),
    };
}

fn main() {
    let mut total_priority: i64 = 0;
    let mut buffer = Vec::new();
    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                buffer.push(ip);
            }
            if buffer.len() == 3 {
                total_priority += get_priority(&buffer) as i64;
                buffer.clear();
            }
        }
    }
    print!("{}\n", total_priority);
}
