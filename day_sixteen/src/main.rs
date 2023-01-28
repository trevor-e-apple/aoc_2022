use std::{collections::HashMap, env, fs::read_to_string, process};

use regex::Regex;

#[derive(Debug)]
struct Valve {
    leads_to: Vec<String>,
    flow_rate: i32,
}

#[derive(Debug, PartialEq, Clone)]
enum ActionType {
    MoveTo,
    Release,
}

#[derive(Debug, Clone)]
struct Action<'a> {
    valve_name: &'a str,
    action_type: ActionType,
    time_left: i32,
    history: Vec<Action<'a>>,
}

fn has_released(history: &Vec<Action>, valve_name: &str) -> bool {
    for action in history {
        if action.action_type == ActionType::Release
            && action.valve_name == valve_name
        {
            return true;
        }
    }

    false
}

fn add_connections_to_stack<'a>(
    stack: &mut Vec<Action<'a>>,
    from: &str,
    name_to_valve: &'a HashMap<String, Valve>,
    time_left: i32,
    history: Vec<Action<'a>>,
) {
    let leads_to = &name_to_valve.get(from).unwrap().leads_to;
    for tunnel_name in leads_to {
        stack.push(Action {
            valve_name: &tunnel_name[0..],
            action_type: ActionType::MoveTo,
            time_left: time_left,
            history: history.clone(),
        });
    }
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

    // SECTION: find all actions you can possibly take
    {
        let time_left = 10;
        let start_from = "AA";
        let mut stack: Vec<Action> = vec![Action {
            valve_name: &start_from[0..],
            action_type: ActionType::Release,
            time_left: time_left,
            history: vec![],
        }];
        // finish initializing stack
        add_connections_to_stack(
            &mut stack,
            &start_from,
            &name_to_valve,
            time_left,
            vec![],
        );
        loop {
            let current_action = match stack.pop() {
                Some(value) => value,
                None => break,
            };
            let mut new_history = current_action.history.clone();
            new_history.push(current_action.clone());

            let time_left = current_action.time_left - 1;
            if time_left > 0 {
                // only try releasing the current valve if we haven't done that
                if (current_action.action_type == ActionType::MoveTo)
                    && !has_released(
                        &current_action.history,
                        &current_action.valve_name[0..],
                    )
                {
                    stack.push(Action {
                        valve_name: current_action.valve_name,
                        action_type: ActionType::Release,
                        time_left: time_left,
                        history: new_history.clone(),
                    });
                }

                // always add moving to other valves as an option
                add_connections_to_stack(
                    &mut stack,
                    current_action.valve_name,
                    &name_to_valve,
                    time_left,
                    new_history.clone(),
                );
            }
        }
    }

    // find how much pressure each path releases
}
