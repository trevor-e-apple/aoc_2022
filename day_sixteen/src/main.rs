use std::{collections::HashMap, env, fs::read_to_string, process, borrow::Borrow};

use regex::Regex;

#[derive(Debug)]
struct Valve {
    name: String,
    valve_index: usize,
    leads_to: Vec<String>,
    flow_rate: i32,
}

struct Vertex<'a> {
    name: &'a str,
    valve_index: usize,
    release_value: i32,
    leads_to: Vec<&'a Vertex<'a>>,
}

type ReleaseState = u32;

#[inline(always)]
fn release_valve(old_state: ReleaseState, valve_index: u32) -> ReleaseState {
    old_state | valve_index
}

#[inline(always)]
fn is_released(state: ReleaseState, valve_index: u32) -> bool {
    (state & valve_index) > 0
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        process::exit(1);
    }

    let path = &args[1];

    // SECTION: parse puzzle input
    let mut valves: Vec<Valve> = Vec::new();
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

            valves.push(Valve {
                name: name.to_string(),
                valve_index: valves.len(),
                leads_to: leads_to,
                flow_rate: flow_rate.parse().unwrap(),
            });
        }

        for valve in &valves {
            println!("{:?}", valve);
        }
    }

    // data about the simulation
    const TIME_LIMIT: i32 = 30;
    const START_FROM_NAME: &str = "AA";

    // initialize graphs with zero-minute data
    let graphs: Vec<HashMap<ReleaseState, Vec<Vertex>>> = {
        // find the start valve
        let mut start_valve: Option<&Valve> = None;
        for valve in &valves {
            if valve.name == START_FROM_NAME {
                start_valve = Some(valve);
                break;
            }
        }

        // initialize zero_minute_graph
        let start_valve = start_valve.unwrap();
        let mut zero_minute_graph: HashMap<ReleaseState, Vec<Vertex>> =
            HashMap::new();
        zero_minute_graph.insert(
            0,
            vec![Vertex {
                name: &start_valve.name,
                valve_index: start_valve.valve_index,
                release_value: (TIME_LIMIT - 1) * start_valve.flow_rate,
                leads_to: vec![],
            }],
        );

        vec![zero_minute_graph]
    };

    // generate all future graphs
    for minute in 1..(TIME_LIMIT + 1) {
        let prev_minute_graphs = graphs.get((minute - 1) as usize).unwrap();
        let mut current_minute_graphs: HashMap<ReleaseState, Vec<Vertex>> =
            HashMap::new();

        for (prev_release_state, prev_vertices) in prev_minute_graphs {
            for prev_vertex in prev_vertices {
                let prev_valve =
                    valves.get(prev_vertex.valve_index as usize).unwrap();

                // vertex hasn't been released in this state
                if !is_released(
                    *prev_release_state,
                    prev_vertex.valve_index as u32,
                ) {
                    let new_release_state = release_valve(
                        *prev_release_state,
                        prev_vertex.valve_index as u32,
                    );

                    // check if there is already a graph for this release state
                    let vertices =
                        match current_minute_graphs.get_mut(&new_release_state) {
                            Some(vertices) => vertices,
                            None => {
                                // create new vertices
                                let vertices = Vec::new();
                                current_minute_graphs
                                    .insert(new_release_state, vertices);
                                current_minute_graphs
                                    .get_mut(&new_release_state)
                                    .unwrap()
                            }
                        };

                    // if the release vertex does not already exist, add it
                    let mut release_vertex_exists = false;
                    for vertex in &mut *vertices {
                        if vertex.valve_index == prev_vertex.valve_index {
                            release_vertex_exists = true;
                            break;
                        }
                    }
                    if !release_vertex_exists {
                        vertices.push(Vertex {
                            name: prev_vertex.name,
                            valve_index: prev_vertex.valve_index,
                            release_value: (TIME_LIMIT - minute)
                                * prev_valve.flow_rate,
                            leads_to: vec![],
                        });
                    }
                    todo!("Point prev_vertex to new release vertex");
                }

                // add the "move" vertices
            }
        }
    }
}
