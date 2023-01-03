use std::{env, fs::read_to_string, process};

#[derive(Debug)]
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

        let mut stack_one: Vec<&Header> = Vec::new();
        // add the first header onto stack_one
        stack_one.push(&list_one.headers[0]);
        // track when we'll need to push the next header
        let next_header_one_start = if list_one.headers.len() > 1 {
            list_one.headers[1].start as i32
        } else {
            -1
        };

        // add the first header onto stack two
        let mut stack_two: Vec<&Header> = Vec::new();
        stack_two.push(&list_two.headers[0]);
        let next_header_two_start = if list_two.headers.len() > 1 {
            list_two.headers[1].start as i32
        } else {
            -1
        };

        // let right_order = loop {
        //     let header_one = &stack_one[0];
        //     let header_two = &stack_two[0];

        //     let one_is_int = ((header_one.start + 1) == header_one.stop);
        //     let two_is_int = ((header_two.start + 1) == header_two.stop);
        //     if one_is_int && two_is_int {
        //         // both values are integers
        //         let element_one = list_one.elements[header_one.start];
        //         let element_two = list_two.elements[header_two.start];
        //         if element_one < element_two {
        //             break true;
        //         } else if element_one > element_two {
        //             break false;
        //         } else {
        //             stack_one.pop().unwrap();
        //             stack_two.pop().unwrap();
        //         }
        //     } else {
        //         // if one_is_int {

        //         // }  else if two_is_int {
        //         //     // only two is an int
        //         // }
        //         for one_index in header_one.start..header_one.stop {
        //             list_one.elements[one_index]
        //         }
        //     }
        // };

        let right_order: bool = loop {
            let header_one = &stack_one[stack_one.len() - 1];
            let header_two = &stack_two[stack_two.len() - 1];

            let mut right_order: Option<bool> = None;

            for index in header_one.start..header_one.stop {
                // check that we don't need to break for a new list
                if (index as i32) == next_header_one_start {
                    // TODO: add new header to the stack
                    break;
                } else {
                    let element_one = list_one.elements[index];
                    let element_two = list_two.elements[index];

                    if element_one < element_two {
                        right_order = Some(true);
                        break;
                    } else if element_two > element_one {
                        right_order = Some(false);
                        break;
                    }
                }
            }

            match right_order {
                Some(value) => break value,
                None => {
                    // we reached the end of comparing two lists, pop those off
                    // -- as we are now done with them
                    stack_one.pop().unwrap();
                    stack_two.pop().unwrap();
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
