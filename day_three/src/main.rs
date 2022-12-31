use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        process::exit(1);
    }

    let path = &args[1];

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut shared_characters: Vec<char> = Vec::new();
    for line in contents.lines() {
        let halfway = (line.len() - 1) / 2;
        let first_half = &line[0..halfway];
        let second_half = &line[halfway..line.len()];

        let mut shared_character: Option<char> = None;
        for first_character in first_half.chars() {
            for second_character in second_half.chars() {
                if first_character == second_character {
                    shared_character = Some(first_character);
                    break;
                }
            }
        }

        match shared_character {
            None => {}
            Some(value) => shared_characters.push(value),
        }
    }

    for shared_character in shared_characters {

        let priority = if shared_character.is_lowercase() {
            u32::from(shared_character) - u32::from('a') + 1
        } else {
            u32::from(shared_character) - u32::from('A') + 27
        };

        println!(
            "{}, priority = {}",
            shared_character,
            priority
        );
    }
}
