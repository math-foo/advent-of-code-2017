use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let filename = "input";
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");


    let chars_to_trim: &[char] = &[' ', '\n', ','];
    let trimmed_str: &str = contents.trim_matches(chars_to_trim);
    let mut registers = HashMap::new();

    for entry in trimmed_str.lines() {
        parse_entry(entry, &mut registers);
    }

    for entry in trimmed_str.lines() {
        let cond_name = entry.split_whitespace().nth(4).unwrap();
        let cond_value = *registers.get(cond_name).unwrap();
        let cond_type = entry.split_whitespace().nth(5).unwrap();
        let const_value: i32 = entry.split_whitespace().nth(6).unwrap().parse().unwrap();
        let mut cond_result = false;
        match cond_type {
            "==" => cond_result = cond_value == const_value,
            "!=" => cond_result = cond_value != const_value,
            ">=" => cond_result = cond_value >= const_value,
            "<=" => cond_result = cond_value <= const_value,
            ">" => cond_result = cond_value > const_value,
            "<" => cond_result = cond_value < const_value,
            _ => println!("uh-oh!"),
        }
        if cond_result {
            let action_name = entry.split_whitespace().nth(0).unwrap();
            let action_type = entry.split_whitespace().nth(1).unwrap();
            let action_value: i32 = entry.split_whitespace().nth(2).unwrap().parse().unwrap();
            let final_value = registers.entry(action_name.to_string()).or_insert(0);

            match action_type {
                "inc" => *final_value += action_value,
                "dec" => *final_value -= action_value,
                _ => println!("uh-oh!"),
            }
        }
    }

    let max_register = registers.values().max().unwrap();
    println!("{}", max_register);
}

fn parse_entry(entry: &str, registers: &mut HashMap<String, i32>) {
    let name: String = entry.split_whitespace().nth(0).unwrap().to_string();
    let other_name: String = entry.split_whitespace().nth(4).unwrap().to_string();

    registers.insert(name, 0);
    registers.insert(other_name, 0);
}


