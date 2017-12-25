use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug,Clone)]
struct Move {
    write: u32,
    move_index: i32,
    state: char
}

#[derive(Debug)]
struct State {
    id: char,
    move_0: Move,
    move_1: Move
}

fn main() {
    let filename = "input";
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    let mut lines = contents.lines();
    let start_state_str = lines.next().unwrap();
    let start_state = start_state_str.split(' ').last().unwrap().chars().nth(0).unwrap();

    let diag_step_count_str = lines.next().unwrap();
    let diag_step_count: u32 = diag_step_count_str.split(' ').nth(5).unwrap().parse().unwrap();

    // skip line
    lines.next();

    let mut state_machine = HashMap::new();

    while lines.clone().count() > 0 {
      let name_of_state_str = lines.next().unwrap();
      let name_of_state = name_of_state_str.split(' ').last().unwrap().chars().nth(0).unwrap();
      // if current value is 0
      lines.next();
      let write_value_0_str = lines.next().unwrap();
      let write_value_0 = write_value_0_str.split(' ').last().unwrap().chars().nth(0).unwrap().to_digit(10).unwrap();
      let move_0_str = lines.next().unwrap();
      let move_0_raw = move_0_str.split(' ').last().unwrap();
      let move_0 = dir_to_int(move_0_raw);
      let next_state_0_str = lines.next().unwrap();
      let next_state_0 = next_state_0_str.split(' ').last().unwrap().chars().nth(0).unwrap();

      let new_move_0 = Move {
        write: write_value_0,
        move_index: move_0,
        state: next_state_0
      };

      // if current value is 1
      lines.next();
      let write_value_1_str = lines.next().unwrap();
      let write_value_1 = write_value_1_str.split(' ').last().unwrap().chars().nth(0).unwrap().to_digit(10).unwrap();
      let move_1_str = lines.next().unwrap();
      let move_1_raw = move_1_str.split(' ').last().unwrap();
      let move_1 = dir_to_int(move_1_raw);
      let next_state_1_str = lines.next().unwrap();
      let next_state_1 = next_state_1_str.split(' ').last().unwrap().chars().nth(0).unwrap();

      let new_move_1 = Move {
        write: write_value_1,
        move_index: move_1,
        state: next_state_1
      };

      let new_state = State {
        id: name_of_state,
        move_0: new_move_0,
        move_1: new_move_1
      };

      state_machine.insert(name_of_state, new_state);

      lines.next();
    }

    let mut tape = HashMap::new();
    let mut cur_index: i32 = 0;
    let mut cur_state = start_state;

    for _ in 0..diag_step_count {
      let cur_value = *tape.entry(cur_index).or_insert(0);
      let state = state_machine.get(&cur_state).unwrap();
      let mut cur_move = state.move_0.clone();

      if cur_value == 1 {
        cur_move = state.move_1.clone();
      }

      tape.insert(cur_index, cur_move.write);
      cur_index += cur_move.move_index;
      cur_state = cur_move.state;
    }

    let result: u32 = tape.values().sum();
    println!("{}", result);
}

fn dir_to_int(dir: &str) -> i32 {
  if dir == "right." {
    1
  } else {
    -1
  }
}
