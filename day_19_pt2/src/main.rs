use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq,Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
struct TravelState {
    row: usize,
    col: usize,
    direction: Direction,
}

impl TravelState {
    fn index(&self) -> usize {
        self.row * 1000 + self.col
    }
}

fn main() {
    let filename = "input";
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let mut entry_hash = HashMap::new();

    for (row, line) in contents.split('\n').enumerate() {
        for (col, entry) in line.chars().enumerate() {
            let index = row * 1000 + col;
            if entry != ' ' {
                entry_hash.insert(index, entry);
            }
        }
    }

    let mut travel_state = TravelState {
        row: 0,
        col: find_start_col(&entry_hash),
        direction: Direction::Down,
    };
    let mut finished = false;
    let mut steps_taken = 0;

    while !finished {
        steps_taken += 1;
        let simple_step = take_step(&entry_hash, &mut travel_state);

        if !simple_step {
            let turn_step = turn_step(&entry_hash, &mut travel_state);

            if turn_step {
                let turned_step = take_step(&entry_hash, &mut travel_state);
                if !turned_step {
                    finished = true;
                }
            } else {
                finished = true;
            }
        }
    }

    println!("{}", steps_taken);
}

fn find_start_col(entries: &HashMap<usize,char>) -> usize {
    let mut result = 0;
    for i in 0..200 {
        let i_usize = i as usize;
        if result == 0 && entries.contains_key(&i_usize) {
            result = i_usize;
        }
    }

    result
}

fn take_step(entries: &HashMap<usize,char>, travel: &mut TravelState) -> bool {
    let mut next_row = travel.row;
    let mut next_col = travel.col;
    let mut hit_edge = false;

    match travel.direction {
        Direction::Down => {
            next_row +=1;
            if next_row > 200 {
                hit_edge = true;
            }
        },
        Direction::Up => {
            if next_row == 0 {
                hit_edge = true
            } else {
                next_row = ((next_row as i32) - 1) as usize
            }
        },
        Direction::Right => {
            next_col += 1;
            if next_col > 200 {
                hit_edge = true;
            }
        }
        Direction::Left => {
            if next_col == 0 {
                hit_edge = true
            } else {
                next_col = ((next_col as i32) - 1) as usize
            }
        },
    }

    if hit_edge {
        return false
    } else {
        let new_index = next_row * 1000 + next_col;
        let new_entry = entries.get(&new_index);

        if new_entry == None {
            return false
        } else {
            travel.row = next_row;
            travel.col = next_col;
            return true
        }
    }
}

fn turn_step(entries: &HashMap<usize,char>, travel: &mut TravelState) -> bool {
    let next_row = travel.row;
    let next_col = travel.col;
    let current_index = travel.index();
    let current_value = *entries.get(&current_index).unwrap();

    if current_value != '+' {
        return false;
    }

    if travel.direction == Direction::Right || travel.direction == Direction::Left {
        if next_row > 0 {
            let up_index = (next_row - 1 as usize) * 1000 + next_col;
            let up_value = entries.get(&up_index);
        
            if up_value != None {
                travel.direction = Direction::Up;
                return true
            }
        }

        if next_row < 200 {
            let down_index = (next_row + 1) * 1000 + next_col;
            let down_value = entries.get(&down_index);
        
            if down_value != None {
                travel.direction = Direction::Down;
                return true
            }
        }
    } else {
        if next_col > 0 {
            let left_index = next_row * 1000 + (next_col - 1 as usize);
            let left_value = entries.get(&left_index);
        
            if left_value != None {
                travel.direction = Direction::Left;
                return true
            }
        }

        if next_col < 200 {
            let right_index = next_row * 1000 + next_col + 1;
            let right_value = entries.get(&right_index);
        
            if right_value != None {
                travel.direction = Direction::Right;
                return true
            }
        }
    }

    return false
}
