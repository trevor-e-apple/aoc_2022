use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        process::exit(1);
    }

    let path = &args[1];
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.lines() {
        for index in 3..line.len() {
            let start_index = index - 3;
            let end_index = index + 1;

            let last_four = &line[start_index..end_index];

            // look at the last four characters, see if there's a duplicate char
            let mut duplicate_found = false;
            for check_index in 1..last_four.len() {
                for search_index in 0..check_index {
                    if (search_index != check_index)
                        && (last_four[check_index..check_index + 1]
                            == last_four[search_index..search_index + 1])
                    {
                        duplicate_found = true;
                        break;
                    }
                }
                if duplicate_found {
                    break;
                }
            }

            // if no duplicate was found in the last four, then we found what
            // -- we needed for this line
            if !duplicate_found {
                println!("{:?}..{:?}", start_index, end_index);
                println!("{:?}", last_four);
                break;
            }
        }
    }
}
