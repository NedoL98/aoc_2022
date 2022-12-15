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
fn get_priority(rucksack: String) -> u8 {
    let (l_compartment, r_compartment) = rucksack
        .as_bytes()
        .split_at(rucksack.len() / 2);

    let l_compartment_items: HashSet<u8> = l_compartment.iter().copied().collect();
    let r_compartment_items: HashSet<u8> = r_compartment.iter().copied().collect();

    let intersection: HashSet<&u8> = 
        l_compartment_items.intersection(&r_compartment_items).collect();
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
    if let Ok(lines) = read_lines("input/test_input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                total_priority += get_priority(ip) as i64;
            }
        }
    }
    print!("{}\n", total_priority);
}
