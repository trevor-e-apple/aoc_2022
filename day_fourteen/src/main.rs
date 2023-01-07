mod rock;

use rock::Rock;
use std::{env, fs::read_to_string, process, thread::sleep, time::Duration};

#[derive(PartialEq, Eq)]
struct Sand {
    pos: (i32, i32),
}

// DOWN IS POSITIVE FOR Y (easier to draw)

/// prints a 20x20 area below the current grain of sand
fn print_state(sand: &Sand, all_rocks: &Vec<Rock>, all_sand: &Vec<Sand>) {
    let start_x = sand.pos.0 - 10;
    let start_y = sand.pos.1;
    let end_x = sand.pos.0 + 10;
    let end_y = sand.pos.1 + 20;

    println!("");
    for y in start_y..end_y {
        for x in start_x..end_x {
            if (x, y) == sand.pos {
                print!("+");
                continue;
            }

            let mut is_rock  = false;
            for rock in all_rocks {
                if rock.check_rock((x, y)) {
                    print!("#");
                    is_rock = true;
                    break;
                }
            }
            if is_rock {
                continue;
            }

            let mut is_sand = false;
            for settled_sand in all_sand {
                if (x, y) == settled_sand.pos {
                    print!("O");
                    is_sand = true;
                    break;
                }
            }
            if is_sand {
                continue;
            }

            print!(".");
        }
        print!("\n");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        process::exit(1);
    }

    let path = &args[1];
    let contents = read_to_string(path).unwrap();

    let mut rocks: Vec<Rock> = Vec::new();
    for line in contents.lines() {
        let mut new_rock = Rock { ..Default::default() };
        for pair in line.split(" -> ") {
            let pair: Vec<&str> = pair.split(",").collect();
            assert!(pair.len() == 2);

            let x: i32 = pair[0].parse().unwrap();
            let y: i32 = pair[1].parse().unwrap();
            new_rock.points.push((x, y));
        }

        rocks.push(new_rock);
    }

    println!("{:?}", rocks);


    const SAND_START: (i32, i32) = (500, 0);
    let mut all_sand: Vec<Sand> = Vec::new();

    print_state(&Sand { pos: SAND_START }, &rocks, &all_sand);

    loop {
        let mut sand = Sand { pos: SAND_START };
        loop {
            // TODO: check for falling into the abyss
            // TODO: sand flow
            let next_pos = (sand.pos.0, sand.pos.1 + 1);

            // check if sand hit another piece of sand
            let mut hit_sand = false;
            for check_sand in &all_sand {
                if check_sand.pos == next_pos {
                    hit_sand = true;
                    break;
                }
            }

            // check if sand hit a rock
            let mut hit_rock = false;
            for rock in &rocks {
                if rock.check_rock(next_pos) {
                    hit_rock = true;
                    break;
                }
            }

            if !(hit_sand || hit_rock) {
                sand.pos = next_pos;
            } else {
                break;
            }

           print_state(&sand, &rocks, &all_sand);
           sleep(Duration::from_secs(1));
        }

        if sand.pos == SAND_START {
            // could not move the sand
            break;
        } else {
            all_sand.push(sand);
        }
    }
}
