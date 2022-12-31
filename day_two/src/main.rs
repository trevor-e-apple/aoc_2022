use std::env;
use std::io::Read;
use std::process;
use std::fs::File;

fn main() {
    const WIN_POINTS: i32 = 6;
    const DRAW_POINTS: i32 = 3;

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        process::exit(1);
    }

    let path = &args[1];

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut score: i32 = 0;
    for line in contents.lines() {
        let round_strategy: Vec<&str> = line.split(" ").collect();
        let opp_choice = round_strategy[0];
        let self_choice = round_strategy[1];

        if self_choice == "X" {
            score += 1;

            if opp_choice == "A" {
                score += DRAW_POINTS;
            } else if opp_choice == "C" {
                score += WIN_POINTS;
            }
        } else if self_choice == "Y" {
            score += 2;

            if opp_choice == "A" {
                score += WIN_POINTS;
            } else if opp_choice == "B" {
                score += DRAW_POINTS;
            }
        } else {
            score += 3;

            if opp_choice == "B" {
                score += WIN_POINTS;
            } else if opp_choice == "C" {
                score += DRAW_POINTS;
            }
        }
    }
    println!("Score: {}", score);
}
