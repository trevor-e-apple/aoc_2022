use std::{env, fs::File, io::Read, process};

fn get_height(heights: &Vec<i32>, row: i32, col: i32, col_count: i32) -> i32 {
    *heights.get((row * col_count + col) as usize).unwrap()
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

    let lines: Vec<&str> = contents.lines().collect();
    let mut tree_row_count = 0;
    let mut tree_col_count = 0;

    let mut heights = Vec::<i32>::new();
    for line in lines.iter() {
        if tree_col_count == 0 {
            for string_height in line.split(" ") {
                heights.push(string_height.parse().unwrap());
                tree_col_count += 1;
            }
        } else {
            for string_height in line.split(" ") {
                heights.push(string_height.parse().unwrap());
            }
        }
        tree_row_count += 1;
    }

    println!("{:?}", heights);

    let mut visible_count = 0;
    for i in 0..tree_row_count {
        if i == 0 || i == (tree_row_count - 1) {
            visible_count += tree_col_count;
        } else {
            // first and last trees in the row are always visible
            visible_count += 2;
            for j in 1..(tree_col_count - 1) {
                let current_height = get_height(&heights, i, j, tree_col_count);

                // check all before in the row
                let mut visible = true;
                for before_index in 0..j {
                    let compare_to =
                        get_height(&heights, i, before_index, tree_col_count);
                    if current_height <= compare_to {
                        visible = false;
                        break;
                    }
                }
                if visible {
                    visible_count += 1;
                } else {
                    // check all after in the row
                    visible = true;
                    for after_index in (j + 1)..tree_col_count {
                        let compare_to = get_height(
                            &heights,
                            i,
                            after_index,
                            tree_col_count,
                        );
                        if current_height <= compare_to {
                            visible = false;
                            break;
                        }
                    }
                    if visible {
                        visible_count += 1;
                    } else {
                        // check all above in the col
                        visible = true;
                        for before_index in 0..i {
                            let compare_to = get_height(
                                &heights,
                                before_index,
                                j,
                                tree_col_count,
                            );
                            if current_height <= compare_to {
                                visible = false;
                                break;
                            }
                        }
                        if visible {
                            visible_count += 1;
                        } else {
                            // check all below in the col
                            visible = true;
                            for after_index in (i + 1)..tree_row_count {
                                let compare_to = get_height(
                                    &heights,
                                    after_index,
                                    j,
                                    tree_col_count,
                                );
                                if current_height <= compare_to {
                                    visible = false;
                                    break;
                                }
                            }
                            if visible {
                                visible_count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{:?}", visible_count);
}
