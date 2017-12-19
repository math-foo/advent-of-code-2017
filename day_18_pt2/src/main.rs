use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::char;

#[derive(Clone,Copy,Debug)]
enum Entry {
    Register(char),
    Value(i64)
}

#[derive(Clone,Copy,Debug)]
enum Instructions {
    Sending(Entry),
    Set(char,Entry),
    Add(char,Entry),
    Multiply(char,Entry),
    Modulo(char,Entry),
    Recover(char),
    Jump(Entry,Entry)
}

#[derive(Debug)]
struct ProgramState {
    program_counter: i64,
    send_counter: i64,
    buffer: VecDeque<i64>,
    registers: HashMap<char, i64>,
    waiting: bool,
    code_length: i64
}

impl ProgramState {
    fn halted(&self) -> bool {
        self.waiting || self.program_counter == self.code_length
    }

    fn send_value(&mut self, v: i64) {
        self.buffer.push_back(v);
        self.waiting = false;
    }

    fn value_at_entry(&mut self, e: Entry) -> i64 {
        match e {
            Entry::Value(i) => return i,
            Entry::Register(c) => {
              let value_at_c = *self.registers.entry(c).or_insert(0);
              return value_at_c;
            }
        }
    }

    fn set(&mut self, reg: char, value: i64) {
        self.registers.insert(reg, value);
    }

    fn add(&mut self, reg: char, value: i64) {
        let value_at_reg = *self.registers.entry(reg).or_insert(0);
        self.registers.insert(reg, value_at_reg + value);
    }

    fn multiply(&mut self, reg: char, value: i64) {
        let value_at_reg = *self.registers.entry(reg).or_insert(0);
        self.registers.insert(reg, value_at_reg * value);
    }

    fn modulo(&mut self, reg: char, value: i64) {
        let value_at_reg = *self.registers.entry(reg).or_insert(0);
        self.registers.insert(reg, value_at_reg % value);
    }

    fn recover(&mut self, reg: char) {
        if self.buffer.is_empty() {
            self.waiting = true;
        } else {
            self.waiting = false;
            let value = self.buffer.pop_front().unwrap();
            self.registers.insert(reg, value);
        }
    }

    fn jump_value(&mut self, value_0: i64, value_1: i64) -> i64 {
        if value_0 > 0 {
            return value_1
        } else {
            return 1
        }
    }
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
            let sending_entry = make_entry(m.split_whitespace().nth(1).unwrap());
            let new_sending = Instructions::Sending(sending_entry);
            program.push(new_sending);
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
            let jump_entry_dec = make_entry(m.split_whitespace().nth(1).unwrap());
            let jump_entry_value = make_entry(m.split_whitespace().last().unwrap());
            let new_jump = Instructions::Jump(jump_entry_dec, jump_entry_value);
            program.push(new_jump);
        } else {
            println!("missed type!! {}", move_type);
        }
    }

    let code_length = program.iter().len() as i64;
    let mut current_program = build_program(0, code_length);
    let mut other_program = build_program(1, code_length);

    while !(current_program.halted() && other_program.halted()) {
        execute(&mut current_program, &mut other_program, &program);
        execute(&mut other_program, &mut current_program, &program);
    }

    println!("{}", other_program.send_counter);
}

fn make_entry(entry_str: &str) -> Entry {
    let parse_result = entry_str.parse::<i64>();

    match parse_result {
        Ok(i) => return Entry::Value(i),
        _ => return Entry::Register(entry_str.chars().last().unwrap())
    }
}

fn build_program(id: i64, code_length: i64) -> ProgramState {
    let mut registers = HashMap::new();
    registers.insert('p', id);

    ProgramState {
        program_counter: 0,
        send_counter: 0,
        registers,
        buffer: VecDeque::new(),
        waiting: false,
        code_length
    }
}

fn execute(program_state: &mut ProgramState, other_program_state: &mut ProgramState, program: &Vec<Instructions>) {
    if !program_state.halted() {
        let next_instruction = program[program_state.program_counter as usize];

        match next_instruction {
            Instructions::Sending(ref e) => {
                let v = program_state.value_at_entry(*e);
                other_program_state.send_value(v);
                program_state.send_counter += 1;
                program_state.program_counter += 1;
            },
            Instructions::Set(c,ref e) => {
                let v = program_state.value_at_entry(*e);
                program_state.set(c,v);
                program_state.program_counter += 1;
            },
            Instructions::Add(c,ref e) => {
                let v = program_state.value_at_entry(*e);
                program_state.add(c,v);
                program_state.program_counter += 1;
            },
            Instructions::Multiply(c,ref e) => {
                let v = program_state.value_at_entry(*e);
                program_state.multiply(c,v);
                program_state.program_counter += 1;
            },
            Instructions::Modulo(c,ref e) => {
                let v = program_state.value_at_entry(*e);
                program_state.modulo(c,v);
                program_state.program_counter += 1;
            },
            Instructions::Recover(c) => {
                program_state.recover(c);
                if !program_state.waiting {
                    program_state.program_counter += 1;
                }
            },
            Instructions::Jump(ref d,ref e) => {
                let b = program_state.value_at_entry(*d);
                let v = program_state.value_at_entry(*e);
                program_state.program_counter += program_state.jump_value(b, v)
            },
        }
    }
}
