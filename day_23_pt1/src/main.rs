use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::char;

#[derive(Clone,Copy,Debug)]
enum Entry {
    Register(char),
    Value(i64)
}

#[derive(Clone,Copy,Debug)]
enum Instructions {
    Set(char,Entry),
    Subtract(char,Entry),
    Multiply(char,Entry),
    Jump(Entry,Entry)
}

fn main() {
    let filename = "input";
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");


    let chars_to_trim: &[char] = &[' ', '\n'];
    let trimmed_str: &str = contents.trim_matches(chars_to_trim);    

    let mut program = Vec::new();

    for m in trimmed_str.lines() {
        let move_type = m.split_whitespace().nth(0).unwrap();

        if move_type == "set" {
            let set_char = m.split_whitespace().nth(1).unwrap().chars().last().unwrap();
            let set_entry = make_entry(m.split_whitespace().last().unwrap());
            let new_set = Instructions::Set(set_char, set_entry);
            program.push(new_set);
        } else if move_type == "sub" {
            let sub_char = m.split_whitespace().nth(1).unwrap().chars().last().unwrap();
            let sub_entry = make_entry(m.split_whitespace().last().unwrap());
            let new_sub = Instructions::Subtract(sub_char, sub_entry);
            program.push(new_sub);
        } else if move_type == "mul" {
            let mul_char = m.split_whitespace().nth(1).unwrap().chars().last().unwrap();
            let mul_entry = make_entry(m.split_whitespace().last().unwrap());
            let new_mul = Instructions::Multiply(mul_char, mul_entry);
            program.push(new_mul);
        } else if move_type == "jnz" {
            let jump_decide_entry = make_entry(m.split_whitespace().nth(1).unwrap());
            let jump_value_entry = make_entry(m.split_whitespace().last().unwrap());
            let new_jump = Instructions::Jump(jump_decide_entry, jump_value_entry);
            program.push(new_jump);
        } else {
            println!("missed type!! {}", move_type);
        }
    }

    let mut program_counter: i64 = 0;
    let program_length = program.iter().count() as i64;
    let mut registers: HashMap<char,i64> = HashMap::new();
    let mut mul_counter = 0;

    while program_counter < program_length {
        let next_instruction = program[program_counter as usize];

        match next_instruction {
            Instructions::Set(c,ref e) => {
                let v = value_at_entry(*e, &mut registers);
                registers.insert(c,v);
                program_counter += 1;
            },
            Instructions::Subtract(c,ref e) => {
                let v = value_at_entry(*e, &mut registers);
                let value_at_c = *registers.entry(c).or_insert(0);
                registers.insert(c,value_at_c - v);
                program_counter += 1;
            },
            Instructions::Multiply(c,ref e) => {
                let v = value_at_entry(*e, &mut registers);
                let value_at_c = *registers.entry(c).or_insert(0);
                registers.insert(c,v * value_at_c);
                program_counter += 1;
                mul_counter += 1;
            },
            Instructions::Jump(ref d,ref v) => {
                let jump_decide = value_at_entry(*d, &mut registers);
                let jump_value = value_at_entry(*v, &mut registers);
                if jump_decide != 0 {
                    program_counter += jump_value;
                } else {
                    program_counter += 1;
                }
            },
        }
    }

    println!("{}", mul_counter);
}

fn make_entry(entry_str: &str) -> Entry {
    let parse_result = entry_str.parse::<i64>();

    match parse_result {
        Ok(i) => return Entry::Value(i),
        _ => return Entry::Register(entry_str.chars().last().unwrap())
    }
}

fn value_at_entry(e: Entry, registers: &mut HashMap<char,i64>) -> i64 {
    match e {
        Entry::Value(i) => return i,
        Entry::Register(c) => {
            let value_at_c = *registers.entry(c).or_insert(0);
            return value_at_c;
        }
    }
}
