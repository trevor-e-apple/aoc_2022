use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

enum ParseState {
    Initial,
    Params,
}

#[derive(Debug)]
struct Parameters {
    count: usize,
    from: usize,
    to: usize,
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

    // parse the input
    const TOKEN_SIZE: usize = 4;
    let mut lines = contents.lines();

    fn set_init_state<'a, 'b>(
        columns: &'a mut Vec<Vec<&'b str>>,
        line: &'b str,
        stack_count: usize,
    ) -> bool {
        if !line.contains("[") {
            return false;
        }

        let mut index = 0;
        while index < stack_count {
            let name_index = TOKEN_SIZE * index + 1;
            let token = line.get(name_index..name_index + 1).unwrap();
            if token != " " {
                columns[index].insert(0, token);
            }
            index += 1;
        }
        true
    }

    // first figure out how many columns we have
    let first_line = lines.next().unwrap();
    // + 1 b/c the new line is not a part of first_line
    let stack_count: usize = (first_line.len() + 1) / TOKEN_SIZE;
    let mut columns: Vec<Vec<&str>> = Vec::new();
    for _ in 0..stack_count {
        columns.push(Vec::new());
    }

    let mut all_parameters: Vec<Parameters> = Vec::new();

    const PARAM_TOKEN_COUNT: usize = 6;
    let mut current_state = ParseState::Initial;
    set_init_state(&mut columns, first_line, stack_count);
    for line in lines {
        match current_state {
            ParseState::Initial => {
                if !set_init_state(&mut columns, line, stack_count) {
                    current_state = ParseState::Params;
                }
            }
            ParseState::Params => {
                let tokens: Vec<&str> = line.split(" ").collect();
                if tokens.len() == PARAM_TOKEN_COUNT {
                    // subtract one for zero indexing
                    let parameters = Parameters {
                        count: tokens[1].parse().unwrap(),
                        from: tokens[3].parse::<usize>().unwrap() - 1,
                        to: tokens[5].parse::<usize>().unwrap() - 1,
                    };
                    all_parameters.push(parameters);
                }
            }
        }
    }

    println!("Initial state");
    println!("{:?}", columns);
    println!("Instructions");
    println!("{:?}", all_parameters);

    for parameters in all_parameters {
        let move_from: &mut Vec<&str> =
            columns.get_mut(parameters.from).unwrap();
        let mut moving_stack: Vec<&str> = Vec::new();
        for _ in 0..parameters.count {
            let moving_value = move_from.remove(move_from.len() - 1);
            moving_stack.push(moving_value);
        }

        let move_to: &mut Vec<&str> = columns.get_mut(parameters.to).unwrap();
        for value in moving_stack.iter().rev() {
            move_to.push(value);
        }
        println!("{:?}", columns);
    }

    println!("Final state");
    println!("{:?}", columns);
}
