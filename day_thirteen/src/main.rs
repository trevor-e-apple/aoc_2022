use std::{env, fs::read_to_string, process};

#[derive(Debug, Clone, PartialEq)]
struct Header {
    start: usize,
    stop: usize,
}

#[derive(Default, Debug)]
struct MixedList {
    elements: Vec<i32>,
    headers: Vec<Header>,
}

fn add_element(
    elements: &mut Vec<i32>,
    contents: &String,
    current_element_start: usize,
    index: usize,
) -> usize {
    let slice = &contents[current_element_start..index];

    let new_element: i32 = match slice.parse() {
        Ok(value) => value,
        Err(_) => {
            return index + 1;
        }
    };
    elements.push(new_element);
    // new int will begin after this character
    index + 1
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        process::exit(1);
    }

    let path = &args[1];
    let contents = read_to_string(path).unwrap();

    let mut lists: Vec<MixedList> = Vec::new();
    let mut header_stack: Vec<Header> = Vec::new();
    let mut current_list = MixedList { ..Default::default() };
    // need to track start of integers since we are parsing char by char
    let mut current_element_start: usize = 0;

    for (index, character) in contents.chars().enumerate() {
        if character == '[' {
            header_stack.push(Header {
                start: current_list.elements.len(),
                stop: current_list.elements.len(),
            });
            // need to track start of int
            current_element_start = index + 1;
        } else if character == ']' {
            // need to add the element before popping the header off
            current_element_start = add_element(
                &mut current_list.elements,
                &contents,
                current_element_start,
                index,
            );

            let mut header = header_stack.pop().unwrap();
            header.stop = current_list.elements.len();
            current_list.headers.push(header);

            if header_stack.len() == 0 {
                lists.push(current_list);
                current_list = MixedList { ..Default::default() };
            }
        } else if character == ',' {
            current_element_start = add_element(
                &mut current_list.elements,
                &contents,
                current_element_start,
                index,
            );
        }
    }

    // because of the stack based approach to constructing headers
    // -- headers are initially ordered with increasing stops (first ordering)
    // -- and decreasing starts (second ordering)
    // therefore, if we perform a stable sort with the start as the key, then
    // -- everything will be in the correct order for the next part
    for list in lists.iter_mut() {
        list.headers.sort_by_key(|key| key.start);
        println!("{:?}", list);
    }

    for first_index in 0..(lists.len() / 2) {
        let second_index = first_index + 1;
        let list_one = &lists[first_index];
        let list_two = &lists[second_index];

        let mut stack_one: Vec<Header> = Vec::new();
        // add the first header onto stack_one
        stack_one.push(list_one.headers[0].clone());

        // add the first header onto stack two
        let mut stack_two: Vec<Header> = Vec::new();
        stack_two.push(list_two.headers[0].clone());

        let right_order = loop {
            let header_one = match stack_one.pop() {
                Some(header) => header,
                None => break true, // list one ran out first, we're in the right order
            };
            let header_two = match stack_two.pop() {
                Some(header) => header,
                None => break false, // list two ran out first, we're in the wrong order
            };

            let header_one_is_int = header_one.start + 1 == header_one.stop;
            let header_two_is_int = header_two.start + 1 == header_two.stop;
            // compare integers
            if header_one_is_int && header_two_is_int {
                if list_one.elements[header_one.start]
                    < list_two.elements[header_two.start]
                {
                    break true; // list one is less, we're in the right order
                } else if list_one.elements[header_one.start]
                    > list_two.elements[header_two.start]
                {
                    break false; // list one is more, we're in the wrong order
                }
                // otherwise elements are the same, keep looking
            } else {
                // spawn all children
                for index in header_one.start..header_one.stop {
                    let mut element_is_int = true;
                    for header in list_one.headers.as_slice() {
                        if header.start == index && *header != header_one {
                            stack_one.push(header.clone());
                            element_is_int = false;
                            break;
                        }
                    }
                    if element_is_int {
                        stack_one
                            .push(Header { start: index, stop: index + 1 });
                    }
                }
                for index in header_two.start..header_two.stop {
                    let mut element_is_int = true;
                    for header in list_two.headers.as_slice() {
                        if header.start == index && *header != header_two {
                            stack_two.push(header.clone());
                            element_is_int = false;
                            break;
                        }
                    }
                    if element_is_int {
                        stack_two
                            .push(Header { start: index, stop: index + 1 });
                    }
                }
            }
        };

        let pair_index = first_index / 2;
        if right_order {
            println!("Pair {} is in the right order", pair_index);
        } else {
            println!("Pair {} is in the wrong order", pair_index);
        }

        break;
    }
}
