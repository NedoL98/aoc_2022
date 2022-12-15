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

struct Test {
    test_fun: Box<dyn Fn(i64) -> bool>,
    monkey_idx_if_true: usize,
    monkey_idx_if_false: usize,
}

fn generate_empty_test() -> Test {
    Test { 
        test_fun: Box::new(|_x| true), 
        monkey_idx_if_false: 0, 
        monkey_idx_if_true: 0, 
    }
}

struct Monkey {
    items: Vec<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    test: Test, 
    inspected_items: i64,
}

fn generate_empty_monkey() -> Monkey {
    Monkey { 
        items: Vec::new(), 
        operation: Box::new(|x| x), 
        test: generate_empty_test(),
        inspected_items: 0, 
    }
}

fn str_to_i64(token: &str) -> i64 {
    token.parse::<i64>().unwrap()
}

fn split_string(str: &String) -> Vec<&str> {
    str.split([',', ' ', ':']).filter(|x| !x.is_empty()).collect()
}

fn parse_input() -> (Vec<Monkey>, i64) {
    let mut total_remainder: i64 = 1; 
    let mut monkeys: Vec<Monkey> = Vec::new(); 
    
    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(mut ip) = line {
                ip = ip.trim().to_string();
                if ip.starts_with("Monkey") {
                    // Init new monkey
                    monkeys.push(generate_empty_monkey());
                } else if ip.starts_with("Starting") {
                    // Read items
                    let tokens: Vec<&str> = split_string(&ip);
                    if tokens.len() >= 2 {
                        for token in tokens[2..].iter() {
                            monkeys.last_mut().unwrap().items.push(
                                str_to_i64(token));
                        }
                    }
                } else if ip.starts_with("Operation") {
                    // Read operation
                    let tokens: Vec<&str> = split_string(&ip);

                    let mut op: Option<Box<dyn Fn(i64) -> i64>> = None;
                    if tokens.last().unwrap() == &"old" {
                        op = match tokens[tokens.len() - 2] {
                            "+" => Some(Box::new(|x| x + x)),
                            "*" => Some(Box::new(|x| x * x)),
                            _ => panic!("unknown operation")
                        }
                    } else {
                        let modifier = str_to_i64(tokens.last().unwrap());
                        op = match tokens[tokens.len() - 2] {
                            "+" => Some(Box::new(move |x| x + modifier)),
                            "*" => Some(Box::new(move |x| x * modifier)),
                            _ => panic!("unknown operation")
                        }
                    }
                    monkeys.last_mut().unwrap().operation = op.unwrap();
                } else if ip.starts_with("Test") {
                    let tokens: Vec<&str> = split_string(&ip);
                    let last_token = str_to_i64(
                        (*tokens.last().unwrap()).to_owned().as_str());
                    monkeys.last_mut().unwrap().test.test_fun = Box::new(
                        move |x| x % last_token == 0);
                    total_remainder *= last_token as i64;
                } else if ip.starts_with("If true") {
                    let tokens: Vec<&str> = split_string(&ip);
                    monkeys.last_mut().unwrap().test.monkey_idx_if_true = 
                        str_to_i64(tokens.last().unwrap()) as usize;
                } else if ip.starts_with("If false") {
                    let tokens: Vec<&str> = split_string(&ip);
                    monkeys.last_mut().unwrap().test.monkey_idx_if_false = 
                        str_to_i64(tokens.last().unwrap()) as usize;
                }
            }
        }
    }

    (monkeys, total_remainder)
}

fn main() {
    let (mut monkeys, total_remainder)  = parse_input(); 

    for round_idx in 0..10000 {
        for idx in 0..monkeys.len() {
            for item_idx in 0..monkeys[idx].items.len() {
                monkeys[idx].items[item_idx] = 
                    (monkeys[idx].operation)(monkeys[idx].items[item_idx]) 
                    % total_remainder;
                let cur_item = monkeys[idx].items[item_idx];
                let to_monkey = 
                    match (monkeys[idx].test.test_fun)(monkeys[idx].items[item_idx]) {
                        true => monkeys[idx].test.monkey_idx_if_true,
                        false => monkeys[idx].test.monkey_idx_if_false,
                };
                monkeys[to_monkey].items.push(cur_item);
                monkeys[idx].inspected_items += 1;
            }
            monkeys[idx].items.clear();
        }
    }

    monkeys.sort_by(
        |lhs, rhs| rhs.inspected_items.cmp(&lhs.inspected_items));

    println!("{}", monkeys[0].inspected_items * monkeys[1].inspected_items);
}