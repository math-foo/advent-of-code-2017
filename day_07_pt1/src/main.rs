use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let filename = "input";
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");


    let chars_to_trim: &[char] = &[' ', '\n', ','];
    let trimmed_str: &str = contents.trim_matches(chars_to_trim);
    let mut programs: Vec<&str> = Vec::new();
    let mut supported_programs = HashSet::new();

    for entry in trimmed_str.lines() {
        let name = entry.split_whitespace().nth(0).unwrap();
        programs.push(name);
        let count = entry.split_whitespace().count();

        if count > 2 {
            let mut index = 3;
            while index < count {
                let raw_supported_name = entry.split_whitespace().nth(index).unwrap();
                let supported_name: &str = raw_supported_name.trim_matches(chars_to_trim);
                supported_programs.insert(supported_name);
                index += 1;
            }
        }
    }

    for program in programs.iter() {
        if !supported_programs.contains(program) {
            println!("{}", program);
        }
    }
}

