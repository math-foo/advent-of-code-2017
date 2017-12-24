use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn turn_right(current: Direction) -> Direction {
    match current {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down
    }
}

fn turn_left(current: Direction) -> Direction {
    match current {
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up
    }
}

fn main() {
    let filename = "input";
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let mut cells = HashMap::new();

    let initial_row_size = contents.lines().count()/2;
    let mut row_index: i32 = initial_row_size as i32;
    for line in contents.lines() {
        let initial_col_size = line.chars().count()/2;
        let mut col_index: i32 = -1 * initial_col_size as i32;

        for entry in line.chars() {
            let entry_index = format!("{}_{}", row_index, col_index);
            cells.insert(entry_index, entry == '#');
            col_index += 1;
        }
        row_index -= 1;
    }

    let mut current_row: i32 = 0; 
    let mut current_col: i32 = 0;
    let mut current_dir = Direction::Up;
    let mut infection_count = 0;

    for _ in 0..10000 {
        let current_index = format!("{}_{}", current_row, current_col);
        let infected = *cells.entry(current_index.clone()).or_insert(false);

        if infected {
            current_dir = turn_right(current_dir);
        } else {
            current_dir = turn_left(current_dir);
        }

        if !infected {
            infection_count += 1;
        }

        cells.insert(current_index, !infected);

        match current_dir {
            Direction::Up => current_row += 1,
            Direction::Down => current_row -= 1,
            Direction::Right => current_col += 1,
            Direction::Left => current_col -= 1,
        }
    }

    println!("{}", infection_count);
}
