use std::{env, process, fs::File, io::Read};

fn add_to_sum(cycle_count: i32, x: i32) -> i32 {
    if ((cycle_count - 20) % 40) == 0 {
        x * cycle_count
    } else {
        0
    }
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

    let mut cycle_count = 1;
    let mut x = 1;
    let mut sum = 0;
    for line in contents.lines() {
        let tokens: Vec<&str> = line.split(" ").collect();
        if tokens[0] == "noop" {
            sum += add_to_sum(cycle_count, x);
            cycle_count += 1;
        } else {
            let operand: i32 = tokens[1].parse().unwrap();
            sum += add_to_sum(cycle_count, x);
            cycle_count += 1;

            sum += add_to_sum(cycle_count, x);
            x += operand;
            cycle_count += 1;
        }
    }

    println!("{:?}", sum);
}
