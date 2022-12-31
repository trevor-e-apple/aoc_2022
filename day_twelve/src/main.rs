use std::{
    collections::{HashSet, VecDeque},
    env,
    fs::read_to_string,
    process,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Start,
    End(i32),
    Path(i32),
}

#[derive(Debug, Default)]
struct Grid {
    dim_x: usize,
    dim_y: usize,
    start_x: usize,
    start_y: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn get_element(&self, x: usize, y: usize) -> Cell {
        assert!(x < self.dim_x);
        assert!(y < self.dim_y);
        self.cells.get(self.dim_x * y + x).unwrap().clone()
    }
}

fn index_to_xy(grid: &Grid, index: usize) -> (usize, usize) {
    let x = index % grid.dim_x;
    let y = index / grid.dim_x;
    (x, y)
}

#[derive(Debug)]
struct QueueEntry {
    cell: Cell,
    path_len: usize,
    x: usize,
    y: usize,
}

fn check_and_add_cell(
    queue: &mut VecDeque<QueueEntry>,
    visited: &mut HashSet<(usize, usize)>,
    grid: &Grid,
    x: i32,
    y: i32,
    from: &QueueEntry,
) -> bool {
    if x < 0 || x >= (grid.dim_x as i32) || y < 0 || (y >= grid.dim_y as i32) {
        return false;
    }

    let x = x as usize;
    let y = y as usize;

    let cell = grid.get_element(x, y);

    // check if adjacent cell is within 1 of current level
    let next_level = match cell {
        Cell::Start => return false, // must have visited, do nothing
        Cell::End(level) => level,
        Cell::Path(level) => level,
    };

    let from_level = match from.cell {
        Cell::Start => 0,
        Cell::End(_) => return false, // should never happen
        Cell::Path(level) => level,
    };

    if i32::abs(next_level - from_level) <= 1 {
        // check if we can end this
        match cell {
            Cell::End(_) => {
                return true;
            }
            _ => {}
        }

        // if we've already visited here, then don't add it to the queue
        if visited.contains(&(x, y)) {
            return false;
        } else {
            visited.insert((x, y));
        }

        queue.push_back(QueueEntry {
            cell: cell,
            path_len: from.path_len + 1,
            x: x,
            y: y,
        });
        return false;
    } else {
        return false;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        process::exit(1);
    }

    let path = &args[1];
    let contents = read_to_string(path).unwrap();

    const A_VALUE: i32 = 97;
    let mut start_index = 0;
    let mut dim_x = 0;
    let mut dim_y = 0;
    let mut grid = Grid {
        ..Default::default()
    };
    for (index, character) in contents.chars().enumerate() {
        let cell = match character {
            'S' => {
                start_index = index;
                Cell::Start
            }
            'E' => Cell::End(26), // end always has value 26 (one more than 'z' - 'a')
            '\n' => {
                grid.dim_x = dim_x;
                dim_x = 0;
                dim_y += 1;
                continue;
            }
            '\r' => {
                continue;
            }
            value => {
                assert!(value.is_alphabetic());
                Cell::Path((u32::from(value) as i32) - A_VALUE)
            }
        };

        dim_x += 1;
        grid.cells.push(cell);
    }

    grid.dim_y = dim_y;
    (grid.start_x, grid.start_y) = index_to_xy(&grid, start_index);

    println!("{:?}", grid);

    // now explore the grid
    // track which cells have been visited already
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    // use a queue b/c we can quit as soon as we find the end
    let mut queue: VecDeque<QueueEntry> = VecDeque::new();
    queue.push_back(QueueEntry {
        cell: grid.get_element(grid.start_x, grid.start_y),
        x: grid.start_x,
        y: grid.start_y,
        path_len: 0,
    });
    // add yourself to the visited set
    visited.insert((grid.start_x, grid.start_y));

    let path_len = loop {
        let entry = match queue.pop_front() {
            None => {
                assert!(false);
                break 0;
            }
            Some(entry) => entry,
        };

        // check the left cell
        let found_end = check_and_add_cell(
            &mut queue,
            &mut visited,
            &grid,
            entry.x as i32 - 1,
            entry.y as i32,
            &entry,
        );
        if found_end {
            break entry.path_len + 1;
        }
        // check the right cell
        let found_end = check_and_add_cell(
            &mut queue,
            &mut visited,
            &grid,
            entry.x as i32 + 1,
            entry.y as i32,
            &entry,
        );
        if found_end {
            break entry.path_len + 1;
        }
        // check the cell above
        let found_end = check_and_add_cell(
            &mut queue,
            &mut visited,
            &grid,
            entry.x as i32,
            entry.y as i32 - 1,
            &entry,
        );
        if found_end {
            break entry.path_len + 1;
        }
        // check the cell below
        let found_end = check_and_add_cell(
            &mut queue,
            &mut visited,
            &grid,
            entry.x as i32,
            entry.y as i32 + 1,
            &entry,
        );
        if found_end {
            break entry.path_len + 1;
        }
    };

    println!("Shortest path: {}", path_len);
}
