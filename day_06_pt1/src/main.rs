use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let filename = "input";
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");


    let chars_to_trim: &[char] = &[' ', '\n'];
    let trimmed_str: &str = contents.trim_matches(chars_to_trim);
    let mut memory_banks: Vec<i32> = Vec::new();

    for entry in trimmed_str.split_whitespace() {
        let x: i32 = entry.trim().parse().unwrap();
        memory_banks.push(x);
    }


    let mut state = memory_banks_state(&memory_banks);
    let mut states_seen = HashSet::new();
    let mut steps = 0;

    while !states_seen.contains(&state) {
        steps += 1;
        states_seen.insert(state);
        redistribute_memory(&mut memory_banks);
        state = memory_banks_state(&memory_banks);
    }

    println!("{}", steps);
}

fn memory_banks_state(memory_banks: &Vec<i32>) -> String {
    let mut new_string: String = String::from("");

    for i in memory_banks.iter() {
        new_string.push_str(&format!("{}_", i));
    }

    new_string
}

fn redistribute_memory(memory_banks: &mut Vec<i32>) {
    let bank_size = memory_banks.len();
    let mut blocks = *memory_banks.iter().max().unwrap();
    let mut index = memory_banks.iter().position(|&i| i == blocks).unwrap();
    memory_banks[index] = 0;
    index += 1;
    if index == bank_size {
        index = 0;
    }

    while blocks > 0 {
        blocks -= 1;
        memory_banks[index] += 1;
        index += 1;
        if index == bank_size {
          index = 0;
        }
    }
}
