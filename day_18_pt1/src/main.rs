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
    Sound(char),
    Set(char,Entry),
    Add(char,Entry),
    Multiply(char,Entry),
    Modulo(char,Entry),
    Recover(char),
    Jump(char,Entry)
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

        if move_type == "snd" {
            let sound_char = m.split_whitespace().last().unwrap().chars().last().unwrap();
            let new_sound = Instructions::Sound(sound_char);
            program.push(new_sound);
        } else if move_type == "set" {
            let set_char = m.split_whitespace().nth(1).unwrap().chars().last().unwrap();
            let set_entry = make_entry(m.split_whitespace().last().unwrap());
            let new_set = Instructions::Set(set_char, set_entry);
            program.push(new_set);
        } else if move_type == "add" {
            let add_char = m.split_whitespace().nth(1).unwrap().chars().last().unwrap();
            let add_entry = make_entry(m.split_whitespace().last().unwrap());
            let new_add = Instructions::Add(add_char, add_entry);
            program.push(new_add);
        } else if move_type == "mul" {
            let mul_char = m.split_whitespace().nth(1).unwrap().chars().last().unwrap();
            let mul_entry = make_entry(m.split_whitespace().last().unwrap());
            let new_mul = Instructions::Multiply(mul_char, mul_entry);
            program.push(new_mul);
        } else if move_type == "mod" {
            let mod_char = m.split_whitespace().nth(1).unwrap().chars().last().unwrap();
            let mod_entry = make_entry(m.split_whitespace().last().unwrap());
            let new_mod = Instructions::Modulo(mod_char, mod_entry);
            program.push(new_mod);
        } else if move_type == "rcv" {
            let recover_char = m.split_whitespace().last().unwrap().chars().last().unwrap();
            let new_recover = Instructions::Recover(recover_char);
            program.push(new_recover);
        } else if move_type == "jgz" {
            let jump_char = m.split_whitespace().nth(1).unwrap().chars().last().unwrap();
            let jump_entry = make_entry(m.split_whitespace().last().unwrap());
            let new_jump = Instructions::Jump(jump_char, jump_entry);
            program.push(new_jump);
        } else {
            println!("missed type!! {}", move_type);
        }
    }

    let mut recovered_sound = false;
    let mut program_counter: i64 = 0;
    let mut registers: HashMap<char,i64> = HashMap::new();
    let mut last_sound = 0;

    while !recovered_sound {
        let next_instruction = program[program_counter as usize];

        match next_instruction {
            Instructions::Sound(c) => {
                let value_at_c = *registers.entry(c).or_insert(0);
                last_sound = value_at_c;
                program_counter += 1;
            },
            Instructions::Set(c,ref e) => {
                let v = value_at_entry(*e, &mut registers);
                registers.insert(c,v);
                program_counter += 1;
            },
            Instructions::Add(c,ref e) => {
                let v = value_at_entry(*e, &mut registers);
                let value_at_c = *registers.entry(c).or_insert(0);
                registers.insert(c,v + value_at_c);
                program_counter += 1;
            },
            Instructions::Multiply(c,ref e) => {
                let v = value_at_entry(*e, &mut registers);
                let value_at_c = *registers.entry(c).or_insert(0);
                registers.insert(c,v * value_at_c);
                program_counter += 1;
            },
            Instructions::Modulo(c,ref e) => {
                let v = value_at_entry(*e, &mut registers);
                let value_at_c = *registers.entry(c).or_insert(0);
                registers.insert(c,value_at_c % v);
                program_counter += 1;
            },
            Instructions::Recover(c) => {
                let value_at_c = *registers.entry(c).or_insert(0);
                if value_at_c != 0 {
                    recovered_sound = true
                }
                program_counter += 1;
            },
            Instructions::Jump(c,ref e) => {
                let v = value_at_entry(*e, &mut registers);
                let value_at_c = *registers.entry(c).or_insert(0);
                if value_at_c > 0 {
                    program_counter += v;
                } else {
                    program_counter += 1;
                }
            },
        }
    }

    println!("{}", last_sound);
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
