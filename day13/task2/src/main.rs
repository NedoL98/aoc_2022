use core::panic;
use std::cmp::Ordering;
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

fn unwrap_list(input: &mut Vec<char>) -> Vec<char> {
    let mut balance = 0;
    assert!(input[0] == '[');

    for idx in 0..input.len() {
        if input[idx] == '[' {
            balance += 1;
        } else if input[idx] == ']' {
            balance -= 1;
        }

        if balance == 0 {
            let (lhs, rhs) = input.split_at(idx + 1);
            let res = lhs[1..idx].to_vec();
            if rhs.is_empty() {
                input.clear();
            }  else {
                *input = rhs[1..].to_vec();
            }
            return res;
        }
    }
    panic!("couldnt find the first token");
}   

fn get_first_token_from_list(input: &mut Vec<char>) -> Vec<char> {
    assert!(input.is_empty() || input[0] != '[');
    if input.is_empty() {
        return vec![];
    }
    
    for idx in 0..input.len() {
        if input[idx] == ',' {
            let (lhs, rhs) = input.split_at(idx);
            let res = lhs.to_vec();
            *input = rhs[1..].to_vec();
            return res;
        }
    }

    let res = input.to_vec();
    input.clear();
    return res;
}

fn get_first_token(input: &mut Vec<char>) -> Vec<char> {
    if !input.is_empty() && input[0] == '[' {
        return unwrap_list(input);
    } else {
        return get_first_token_from_list(input);
    }
}  

fn compare(lhs: &mut Vec<char>, rhs: &mut Vec<char>) -> Option<bool> {
    if lhs.is_empty() && rhs.is_empty() {
        return None;
    } else if lhs.is_empty() && !rhs.is_empty() {
        return Some(true);
    } else if !lhs.is_empty() && rhs.is_empty() {
        return Some(false);
    }
    let mut lhs_item = get_first_token(lhs);
    let mut rhs_item = get_first_token(rhs);
    
    if lhs.is_empty() && rhs.is_empty() && lhs_item.is_empty() && rhs_item.is_empty() {
        return None;
    } else if lhs_item.is_empty() && rhs_item.is_empty() {
        return compare(lhs, rhs);
    } else if lhs_item.is_empty() && !rhs_item.is_empty() {
        return Some(true);
    } else if !lhs_item.is_empty() && rhs_item.is_empty() {
        return Some(false);
    } else if !lhs_item.starts_with(&['[']) 
        && !rhs_item.starts_with(&['[']) 
        && !lhs_item.contains(&',') 
        && !rhs_item.contains(&',') {
        let lhs_num = 
            lhs_item.iter().collect::<String>().parse::<i32>().unwrap();
        let rhs_num = 
            rhs_item.iter().collect::<String>().parse::<i32>().unwrap();
        if lhs_num == rhs_num {
            return compare(lhs, rhs);
        } else {
            Some(lhs_num < rhs_num)
        }
    } else {
        let comp_res = compare(&mut lhs_item, &mut rhs_item);
        if comp_res.is_some() {
            return comp_res;
        }
        compare(lhs, rhs)
    }
}

fn lines_are_disordered(lhs: &mut Vec<char>, rhs: &mut Vec<char>) -> bool {
    return compare(
        &mut get_first_token(lhs), 
        &mut get_first_token(rhs)).unwrap();
}

fn main() {
    let mut input_lines: Vec<Vec<char>> = Vec::new();
    input_lines.push("[[2]]".chars().collect());
    input_lines.push("[[6]]".chars().collect());

    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if !ip.trim().is_empty() {
                    input_lines.push(ip.trim().chars().collect());
                }
            }
        }
    }

    input_lines.sort_by(|lhs, rhs| {
        let mut lhs_copy = lhs.clone();
        let mut rhs_copy = rhs.clone();
        if lines_are_disordered(&mut lhs_copy, &mut rhs_copy) == true {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut answer = 1;
    for idx in 0..input_lines.len() {
        let string = input_lines[idx].iter().collect::<String>();
        if string == "[[2]]" || string == "[[6]]" {
            answer *= idx + 1;
        }
    }
    println!("{}", answer);
}