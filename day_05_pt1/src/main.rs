use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "input";
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");


    let chars_to_trim: &[char] = &[' ', '\n'];
    let trimmed_str: &str = contents.trim_matches(chars_to_trim);
    let mut program: Vec<i32> = Vec::new();

    for entry in trimmed_str.lines() {
        let x: i32 = entry.parse().unwrap();
        program.push(x);
    }

    let mut current_index: i32 = 0;
    let max_index: i32 = program.len() as i32;
    let mut step_count = 0;

    while (current_index > -1) && (current_index < max_index) {
        let next_jump = program[current_index as usize];
        program[current_index as usize] += 1;
        step_count += 1;
        current_index += next_jump;
    }

    println!("{}", step_count);
}
