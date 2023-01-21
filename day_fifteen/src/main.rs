use regex::Regex;
use std::{env, fs::read_to_string, process};

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

fn get_manhattan_distance(a: &Position, b: &Position) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        process::exit(1);
    }

    // make regex for parsing input
    let reggie = Regex::new(concat!(
        r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): ",
        r"closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)"
    ))
    .unwrap();

    let path = &args[1];
    let contents = read_to_string(path).unwrap();

    let mut sensors: Vec<Position> = Vec::new();
    let mut beacons: Vec<Position> = Vec::new();
    for line in contents.lines() {
        let group = reggie.captures(line).unwrap();

        let x = group.name("sensor_x").unwrap().as_str();
        let y = group.name("sensor_y").unwrap().as_str();
        sensors.push(Position { x: x.parse().unwrap(), y: y.parse().unwrap() });

        let x = group.name("beacon_x").unwrap().as_str();
        let y = group.name("beacon_y").unwrap().as_str();
        beacons.push(Position { x: x.parse().unwrap(), y: y.parse().unwrap() });
    }

    println!("sensors");
    println!("{:?}", sensors);
    println!("beacons");
    println!("{:?}", beacons);

    let mut x_min = i32::MAX;
    let mut y_min = i32::MAX;
    let mut x_max = i32::MIN;
    let mut y_max = i32::MIN;

    let mut manhattan_distances: Vec<i32> = Vec::new();
    // figure out the manhattan distance between a sensor and its beacon
    for (sensor, beacon) in sensors.iter().zip(beacons) {
        let manhattan_distance = get_manhattan_distance(sensor, &beacon);
        manhattan_distances.push(manhattan_distance);

        if (sensor.x - manhattan_distance) < x_min {
            x_min = sensor.x - manhattan_distance;
        }
        if (sensor.x + manhattan_distance) > x_max {
            x_max = sensor.x + manhattan_distance;
        }
        if (sensor.y - manhattan_distance) < y_min {
            y_min = sensor.y - manhattan_distance;
        }
        if (sensor.y + manhattan_distance) > y_max {
            y_max = sensor.y + manhattan_distance;
        }
    }

    println!("({:?}, {:?}), ({:?}, {:?})", x_min, y_min, x_max, y_max);
    for x in x_min..x_max {
        for y in y_min..y_max {
            let point = Position { x: x, y: y };
            // find the manhattan distance from point to each sensor
            for (sensor, distance_to_beacon) in
                sensors.iter().zip(&manhattan_distances)
            {
                let distance_to_point = get_manhattan_distance(&sensor, &point);
                if distance_to_point < *distance_to_beacon {
                    println!(
                        "{:?} is within range ({:?}) of sensor {:?}",
                        point, *distance_to_beacon, sensor
                    );
                    break;
                }
            }
            // if within sensor range, do not increment count
        }
    }
}
