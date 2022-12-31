use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

fn get_range_ints(input: &str) -> (i32, i32) {
    let bounds: Vec<&str> = input.split('-').collect();
    let lower_bound = bounds[0].trim().parse::<i32>().unwrap();
    let upper_bound = bounds[1].trim().parse::<i32>().unwrap();
    (lower_bound, upper_bound)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        process::exit(1);
    }

    let path = &args[1];

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for (index, line) in contents.lines().enumerate() {
        let ranges: Vec<&str> = line.split(',').collect();
        let (first_lower_bound, first_upper_bound) = get_range_ints(ranges[0]);
        let (second_lower_bound, second_upper_bound) = get_range_ints(ranges[1]);

        if (first_lower_bound < second_lower_bound && first_upper_bound > second_upper_bound)
            || (second_lower_bound < first_lower_bound && second_upper_bound > first_upper_bound)
        {
            println!("Line {} ({}) has a fully contained range", index, line);
        }
    }
}
