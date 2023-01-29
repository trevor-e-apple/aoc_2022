use std::{collections::HashMap, env, fs::read_to_string, process};

use regex::Regex;

#[derive(Debug)]
struct Valve {
    leads_to: Vec<String>,
    flow_rate: i32,
}

#[derive(Debug)]
struct ValveLimitInfo {
    value: i32,
    already_released: Vec<String>,
}

#[allow(dead_code)]
fn print_just_values(data: &HashMap<String, ValveLimitInfo>) {
    for (name, limit_info) in data {
        print!("{:?}: {:?}, ", name, limit_info.value);
    }
    println!("");
}

#[allow(dead_code)]
fn print_just_aa(data: &HashMap<String, ValveLimitInfo>) {
    print!("{:?}, ", data.get("AA").unwrap());
    println!("");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        process::exit(1);
    }

    let path = &args[1];

    // SECTION: parse puzzle input
    let mut name_to_valve: HashMap<String, Valve> = HashMap::new();
    {
        let contents = read_to_string(path).unwrap();

        let reggie = Regex::new(concat!(
            r"Valve (?P<name>.+) has flow rate=(?P<flow_rate>[0-9]+);",
            r" tunnel[s]? lead[s]? to valve[s]? (?P<leads_to>.+)"
        ))
        .unwrap();
        for line in contents.lines() {
            let group = reggie.captures(line).unwrap();

            let name = group.name("name").unwrap().as_str();
            let flow_rate = group.name("flow_rate").unwrap().as_str();
            let leads_to_string = group.name("leads_to").unwrap().as_str();
            let mut leads_to: Vec<String> = Vec::new();
            for valve_name in leads_to_string.split(",") {
                leads_to.push(valve_name.to_string().trim().to_string());
            }

            name_to_valve.insert(
                name.to_string(),
                Valve {
                    leads_to: leads_to,
                    flow_rate: flow_rate.parse().unwrap(),
                },
            );
        }

        for (name, valve) in &name_to_valve {
            println!("{:?}: {:?}", name, valve);
        }
    }

    let mut release_values: Vec<HashMap<String, ValveLimitInfo>> = {
        // initialize our release values
        let mut zero_minute_values: HashMap<String, ValveLimitInfo> =
            HashMap::new();
        let mut one_minute_values: HashMap<String, ValveLimitInfo> =
            HashMap::new();
        for (name, _) in &name_to_valve {
            zero_minute_values.insert(
                name.to_string(),
                ValveLimitInfo { value: 0, already_released: vec![] },
            );
            one_minute_values.insert(
                name.to_string(),
                ValveLimitInfo { value: 0, already_released: vec![] },
            );
        }
        vec![zero_minute_values, one_minute_values]
    };
    {
        // actually limit + 1 b/c the loop is not inclusive
        const TIME_LIMIT: i32 = 31;

        for current_limit in 2..TIME_LIMIT {
            let two_back_limit_info =
                release_values.get((current_limit - 2) as usize).unwrap();
            let prev_limit_info =
                release_values.get((current_limit - 1) as usize).unwrap();
            let mut current_limit_values: HashMap<String, ValveLimitInfo> =
                HashMap::new();
            for (name, valve) in &name_to_valve {
                // compute how much time the current valve is worth if you
                // -- release it and then move to the best neighbor that hasn't
                // -- released the current valve
                let (release_value, release_move_neighbor) = {
                    let release_value = (current_limit - 1) * valve.flow_rate;

                    let mut best_neighbor_value: i32 = -1;
                    let mut best_neighbor_name: Option<&String> = None;
                    for neighbor_name in &valve.leads_to {
                        let neighbor_info =
                            two_back_limit_info.get(neighbor_name).unwrap();

                        // can only release then move to a neighbor who has not
                        // -- released us as a part of their best path
                        if !neighbor_info.already_released.contains(name)
                            && neighbor_info.value > best_neighbor_value
                        {
                            best_neighbor_value = neighbor_info.value;
                            best_neighbor_name = Some(neighbor_name);
                        }
                    }
                    match best_neighbor_name {
                        Some(neighbor_name) => (
                            release_value + best_neighbor_value,
                            two_back_limit_info.get(neighbor_name),
                        ),
                        None => (release_value, None),
                    }
                };

                // look up how much value you get by moving to one of your
                // -- neighbors without releasing
                let mut move_value: i32 = -1;
                let mut move_neighbor: Option<&ValveLimitInfo> = None;
                {
                    let mut best_neighbor_name: Option<&String> = None;
                    for neighbor_name in &valve.leads_to {
                        let neighbor_info =
                            prev_limit_info.get(neighbor_name).unwrap();
                        if neighbor_info.value > move_value {
                            move_value = neighbor_info.value;
                            best_neighbor_name = Some(neighbor_name);
                        }
                    }

                    match best_neighbor_name {
                        Some(neighbor_name) => {
                            move_neighbor = prev_limit_info.get(neighbor_name);
                        }
                        None => {}
                    };
                }

                let mut already_released: Vec<String> = vec![];
                let best_value = if release_value > move_value {
                    // add self to already_released
                    already_released.push(name.clone());

                    // add best neighbor's already_released
                    match release_move_neighbor {
                        Some(neighbor) => {
                            for released_name in &neighbor.already_released
                            {
                                already_released.push(released_name.clone());
                            }
                        },
                        None => {},
                    }

                    release_value
                } else {
                    let move_neighbor = match move_neighbor {
                        Some(neighbor) => neighbor,
                        None => panic!("oh dear"),
                    };

                    // add best neighbor's already released
                    for released_name in &move_neighbor.already_released {
                        already_released.push(released_name.clone());
                    }

                    move_value
                };
                current_limit_values.insert(
                    name.clone(),
                    ValveLimitInfo {
                        value: best_value,
                        already_released: already_released,
                    },
                );
            }

            release_values.push(current_limit_values);
        }
    }

    // SECTION: find and print the maximum pressure
    {
        for (index, limit_info) in release_values.iter().enumerate() {
            print!("{:?}: ", index);
            print_just_values(limit_info);
        }
        for (index, limit_info) in release_values.iter().enumerate() {
            print!("{:?}: ", index);
            print_just_aa(limit_info);
        }
        let limit_info = release_values.get(release_values.len() - 1).unwrap();
        // always start from AA
        let max_value = limit_info.get("AA").unwrap();
        println!("Max_value: {:?}", max_value);
    }
}
