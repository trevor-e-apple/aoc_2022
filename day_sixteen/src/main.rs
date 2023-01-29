use std::{collections::HashMap, env, fs::read_to_string, process};

use regex::Regex;

#[derive(Debug)]
struct Valve {
    leads_to: Vec<String>,
    flow_rate: i32,
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

    // SECTION: construct our solution
    let mut release_values: Vec<HashMap<String, i32>> = Vec::new();
    {
        // actually limit + 1 b/c the loop is not inclusive
        const TIME_LIMIT: i32 = 7;

        // initialize our release values
        let mut zero_minute_values: HashMap<String, i32> = HashMap::new();
        for (name, _) in &name_to_valve {
            zero_minute_values.insert(name.to_string(), 0);
        }
        release_values.push(zero_minute_values);

        for current_limit in 1..TIME_LIMIT {
            let last_limit_values =
                release_values.get((current_limit - 1) as usize).unwrap();
            let mut current_limit_values: HashMap<String, i32> = HashMap::new();
            for (name, valve) in &name_to_valve {
                // compute how much time the current valve is worth if you
                // -- release it and just sit there
                let mut release_value = (current_limit - 1) * valve.flow_rate;

                // look up how much value you get by moving to one of your
                // -- neighbors
                for neighbor_name in &valve.leads_to {
                    let neighbor_value =
                        *last_limit_values.get(neighbor_name).unwrap();
                    if neighbor_value > release_value {
                        release_value = neighbor_value;
                    }
                }

                current_limit_values.insert(name.clone(), release_value);
            }

            release_values.push(current_limit_values);
        }
    }

    // SECTION: find and print the maximum pressure
    {
        for limit_values in &release_values {
            println!("{:?}", limit_values);
        }

        let limit_values = release_values.get(release_values.len() - 1).unwrap();
        println!("{:?}", limit_values);
        // always start from AA
        let max_value = limit_values.get("AA").unwrap();
        println!("Max_value: {}", max_value);
    }
}
