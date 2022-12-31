use std::{collections::HashSet, env, fs::File, io::Read, process};

#[derive(Debug)]
enum Instruction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

/// gets the direction that we need to move the tail towards the head
fn get_direction(distance: i32) -> i32 {
    if distance == 0 {
        0
    } else if distance < 0 {
        1
    } else {
        -1
    }
}

/// move the tail towards the head
fn move_tail(
    current_head: (i32, i32),
    current_tail: (i32, i32),
    visited: &mut HashSet<(i32, i32)>,
) -> (i32, i32) {
    let x_distance = current_head.0 - current_tail.0;
    let y_distance = current_head.1 - current_tail.1;

    let result = if (x_distance == 1 || x_distance == -1 || x_distance == 0)
        && (y_distance == 1 || y_distance == -1 || y_distance == 0)
    {
        // head is adjacent to tail
        current_tail
    } else {
        let x_direction = get_direction(x_distance);
        let y_direction = get_direction(y_distance);
        (current_tail.0 - x_direction, current_tail.1 - y_direction)
    };

    visited.insert(result);
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        process::exit(1);
    }

    let path = &args[1];

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut instructions: Vec<Instruction> = Vec::new();
    for line in contents.lines() {
        println!("{:?}", line);
        let tokens: Vec<&str> = line.split(" ").collect();
        let distance: i32 = tokens.get(1).unwrap().parse().unwrap();
        let direction = tokens.get(0).unwrap();
        let instruction = match *direction {
            "U" => Instruction::Up(distance),
            "D" => Instruction::Down(distance),
            "L" => Instruction::Left(distance),
            "R" => Instruction::Right(distance),
            _ => {
                assert!(false);
                Instruction::Up(0)
            }
        };

        instructions.push(instruction);
    }

    println!("{:?}", instructions);

    // TODO: statically allocate vector to track visited positions
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut current_head: (i32, i32) = (0, 0);
    let mut current_tail = current_head;
    visited.insert(current_tail);
    for instruction in instructions {
        match instruction {
            Instruction::Up(distance) => {
                for _ in 0..distance {
                    current_head = (current_head.0, current_head.1 + 1);
                    current_tail =
                        move_tail(current_head, current_tail, &mut visited);
                }
            }
            Instruction::Down(distance) => {
                for _ in 0..distance {
                    current_head = (current_head.0, current_head.1 - 1);
                    current_tail =
                        move_tail(current_head, current_tail, &mut visited);
                }
            }
            Instruction::Left(distance) => {
                for _ in 0..distance {
                    current_head = (current_head.0 - 1, current_head.1);
                    current_tail =
                        move_tail(current_head, current_tail, &mut visited);
                }
            }
            Instruction::Right(distance) => {
                for _ in 0..distance {
                    current_head = (current_head.0 + 1, current_head.1);
                    current_tail =
                        move_tail(current_head, current_tail, &mut visited);
                }
            }
        }
    }

    println!("current_head: {:?}", current_head);
    println!("current_tail: {:?}", current_tail);
    println!("Visited: {:?}", visited.len());
}
