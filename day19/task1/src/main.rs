use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::ops::{Add, Sub};
use std::cmp::{max, Ordering};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone, Eq, Hash)]
struct Resource {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl PartialOrd for Resource {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.ore == other.ore
            && self.clay == other.clay
            && self.obsidian == other.obsidian
            && self.geode == other.geode {
            return Some(Ordering::Equal);
        } else if self.ore <= other.ore
            && self.clay <= other.clay
            && self.obsidian <= other.obsidian
            && self.geode <= other.geode {
            return Some(Ordering::Less);
        } else if self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode {
            return Some(Ordering::Greater);
        } else {
            return None;
        }
    }
}

impl PartialEq for Resource {
    fn eq(&self, other: &Self) -> bool {
        return self.ore == other.ore
            && self.clay == other.clay
            && self.obsidian == other.obsidian
            && self.geode == other.geode;
    }
}

impl<'a, 'b> Add<&'b Resource> for &'a Resource {
    type Output = Resource;
    fn add(self, other: &'b Resource) -> Resource {
        Resource { 
            ore: self.ore + other.ore, 
            clay: self.clay + other.clay, 
            obsidian: self.obsidian + other.obsidian, 
            geode: self.geode + other.geode,
        }
    }
}

impl Add<Resource> for Resource {
    type Output = Resource;
    fn add(self, other: Resource) -> Resource {
        Resource { 
            ore: self.ore + other.ore, 
            clay: self.clay + other.clay, 
            obsidian: self.obsidian + other.obsidian, 
            geode: self.geode + other.geode,
        }
    }
}

impl<'a, 'b> Sub<&'b Resource> for &'a Resource {
    type Output = Resource;
    fn sub(self, other: &'b Resource) -> Resource {
        Resource { 
            ore: self.ore - other.ore, 
            clay: self.clay - other.clay, 
            obsidian: self.obsidian - other.obsidian, 
            geode: self.geode - other.geode, 
        }
    }
}

impl Sub<Resource> for Resource {
    type Output = Resource;
    fn sub(self, other: Resource) -> Resource {
        Resource { 
            ore: self.ore - other.ore, 
            clay: self.clay - other.clay, 
            obsidian: self.obsidian - other.obsidian, 
            geode: self.geode - other.geode, 
        }
    }
}

fn generate_default_resource() -> Resource {
    Resource { ore: 0, clay: 0, obsidian: 0, geode: 0 }
}

fn generate_unit_resource(idx: i32) -> Resource {
    match idx {
        0 => Resource { ore: 1, clay: 0, obsidian: 0, geode: 0 },
        1 => Resource { ore: 0, clay: 1, obsidian: 0, geode: 0 },
        2 => Resource { ore: 0, clay: 0, obsidian: 1, geode: 0 },
        3 => Resource { ore: 0, clay: 0, obsidian: 0, geode: 1 },
        _ => panic!("incorrect idx"),
    }
}

fn get_robots_costs(input: String) -> Vec<Resource> {
    let robots_str: Vec<&str> = input.split(['.', ':']).collect();
    let mut robots: Vec<Resource> = Vec::new();
    robots.reserve(robots_str.len() - 1);
    
    let ore_re = Regex::new(r"(\d+) ore").unwrap();    
    let clay_re = Regex::new(r"(\d+) clay").unwrap();    
    let obsidian_re = Regex::new(r"(\d+) obsidian").unwrap();    

    for robot_idx in 1..robots_str.len() - 1 {
        let mut cost = generate_default_resource();
        let robot_str = robots_str[robot_idx];

        let ore_matches = &ore_re.captures(&robot_str);
        if ore_matches.is_some() {
            cost.ore = ore_matches.as_ref().unwrap()[1].parse::<i32>().unwrap();
        }

        let clay_matches = &clay_re.captures(&robot_str);
        if clay_matches.is_some() {
            cost.clay = clay_matches.as_ref().unwrap()[1].parse::<i32>().unwrap();
        }

        let obsidian_matches = &obsidian_re.captures(&robot_str);
        if obsidian_matches.is_some() {
            cost.obsidian = obsidian_matches.as_ref().unwrap()[1].parse::<i32>().unwrap();
        }

        robots.push(cost);
    }

    robots
}

fn go(
    time: i32, 
    robots: Resource, 
    resources: Resource,
    robots_costs: &Vec<Resource>) -> i32 {

    // println!("{}", time);

    if time == 24 {
        return resources.geode;
    }

    let mut can_build_cnt_robots = 0;
    let mut max_geode = 0;     
    
    for (idx, robot_cost) in robots_costs.iter().enumerate() {
        if &resources >= robot_cost {
            max_geode = max(max_geode, go(
                time + 1, 
                &robots + &generate_unit_resource(idx as i32),
                &(&resources - robot_cost) + &robots,
                robots_costs));
            can_build_cnt_robots += 1;
        }
    }
    
    // It is always optimal build some robot 
    // if we can build all of them
    if can_build_cnt_robots < 4 {
        max_geode = max(max_geode, go(
            time + 1, 
            robots.clone(),
            &resources + &robots,
            robots_costs));
    }

    max_geode
}

fn update_set(set: &mut HashSet<Resource>, new_value: Resource) {
    set.retain(|set_elem| !(set_elem <= &new_value) );
    for elem in set.iter() {
        if elem > &new_value {
            return;
        }
    }
    set.insert(new_value);
}

fn get_geode_lower_bound(
    time: i32, 
    mut robots: Resource, 
    mut resources: Resource, 
    geode_cost: &Resource) -> i32 {
    // Only buy geode
    for _ in 0..time {
        if resources >= *geode_cost {
            resources = &(robots.clone() + resources.clone()) - geode_cost;
            robots = robots.clone() + generate_unit_resource(3);
        }
    }
    resources.geode
}

fn get_geode_upper_bound(
    time: i32, robots: &Resource, resources: &Resource) -> i32 {
    return resources.geode 
        + robots.geode * time
        + time * (time - 1) / 2;
}

fn get_answer(input: String) -> i32 {
    let blueprint_idx_re = Regex::new(r"Blueprint (\d*)").unwrap();    
    let blueprint_idx = 
        blueprint_idx_re.captures(&input).unwrap()[1].parse::<i32>().unwrap();

    let robots_costs = get_robots_costs(input);

    // Time -> Robots -> All possible resources
    let mut dp: HashMap<Resource, HashSet<Resource>> = HashMap::from([
        (generate_unit_resource(0), 
        HashSet::from([generate_default_resource()]))
    ]);
    let mut geode_lower_bound = 0;

    for time in 0..24 {
        // println!("{} {}", time, geode_lower_bound);
        let mut next_dp_layer: HashMap<Resource, HashSet<Resource>> = 
            HashMap::new();
        for (robots, resources_set) in &dp {
            for resources in resources_set {
                if get_geode_upper_bound(24 - time, robots, resources) 
                    <= geode_lower_bound {
                    continue;
                }
                let mut can_build_cnt_robots = 0;

                for (idx, robot_cost) in robots_costs.iter().enumerate() {
                    if resources >= robot_cost {
                        let new_robots = robots + &generate_unit_resource(idx as i32);
                        let new_resources = &(resources - robot_cost) + robots;

                        if let Some(new_resources_set)  
                            = next_dp_layer.get_mut(&new_robots) {
                            update_set(new_resources_set, new_resources);
                        } else {
                            next_dp_layer.insert(
                                new_robots, HashSet::from([new_resources]));
                        }
                        can_build_cnt_robots += 1;
                    }
                }

                // It is always optimal to build some robot 
                // if we can build all of them
                if can_build_cnt_robots < 4 {
                    if let Some(resources_set) 
                        = next_dp_layer.get_mut(robots) {
                        update_set(resources_set, resources + robots);
                    } else {
                        next_dp_layer.insert(
                            robots.clone(), HashSet::from([resources + robots]));
                    }
                }
            }
        }

        for (robots, resources_set) in &next_dp_layer {
            for resources in resources_set {
                geode_lower_bound = max(
                    geode_lower_bound, 
                    get_geode_lower_bound(
                        23 - time, 
                        robots.clone(), 
                        resources.clone(), 
                        &robots_costs[3]));
            }
        }

        dp = next_dp_layer;
    }

    // let max_geode = go(
    //     0, 
    //     generate_unit_resource(0), 
    //     generate_default_resource(),
    //     &robots_costs);

    // let mut max_geode = 0;
    // for (_robots, resources_set) in &dp {
    //     for resource in resources_set {
    //         max_geode = max(max_geode, resource.geode);
    //     }
    // }

    println!("complete {}!", blueprint_idx);
    println!("{}", geode_lower_bound);

    return blueprint_idx * geode_lower_bound;
}

fn main() {
    let mut answer = 0;

    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                answer += get_answer(ip);
            }
        }
    }

    println!("{}", answer);
}