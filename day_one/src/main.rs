use std::env;
use std::io::Read;
use std::process;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        process::exit(1);
    }
    
    let path = &args[1];

    let mut file = File::open(path).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut max_elf_index: i32 = 0;
    let mut most_calories: i32 = -1;
    let mut current_elf: i32 = 0;
    let mut current_calories: i32 = 0;
    for line in contents.lines() {
        if line == "" {
            if current_calories > most_calories {
                most_calories = current_calories;
                max_elf_index = current_elf;
            }
            current_calories = 0;
            current_elf += 1;
        }
        else {
            current_calories += line.parse::<i32>().unwrap();
        }
    }

    println!("{}: {}", max_elf_index, most_calories);
}