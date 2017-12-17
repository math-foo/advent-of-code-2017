use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::char;

enum Moves {
    Spin(usize),
    Exchange(usize,usize),
    Swap(char,char)
}

fn main() {
    let filename = "input";
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");


    let chars_to_trim: &[char] = &[' ', '\n'];
    let trimmed_str: &str = contents.trim_matches(chars_to_trim);    

    let program_u32 = 16;
    let program_usize: usize = 16;
    
    let mut program_to_index: HashMap<char,usize> = HashMap::new();
    let mut index_to_program: HashMap<usize,char> = HashMap::new();

    for i in 0..program_u32 {
        program_to_index.insert(char::from_u32(97 + i).unwrap(), i as usize);
        index_to_program.insert(i as usize, char::from_u32(97 + i).unwrap());
    }

    let mut move_vec = Vec::new();

    for m in trimmed_str.split(',') {
        let move_type = m.chars().nth(0).unwrap();

        if move_type == 's' {
            let spin_amount: usize = m.split('s').last().unwrap().parse().unwrap();
            let spin_move = Moves::Spin(spin_amount);
            move_vec.push(spin_move);
        } else if move_type == 'x' {
            let mut exchange_values = m.split('x').last().unwrap().split('/');
            let a: usize = exchange_values.nth(0).unwrap().parse().unwrap();
            let b: usize = exchange_values.last().unwrap().parse().unwrap();
            let exchange_move = Moves::Exchange(a,b);
            move_vec.push(exchange_move);
        } else if move_type == 'p' {
            let a: char = m.chars().nth(1).unwrap();
            let b: char = m.chars().nth(3).unwrap();
            let swap_move = Moves::Swap(a,b);
            move_vec.push(swap_move);
        } else {
            println!("missed type!! {}", move_type);
        }
    }

    let mut spin_offset = 0;

    for m in move_vec.iter() {
        match *m {
            Moves::Spin(n) => {
                spin_offset += (program_usize - n) as usize;
                spin_offset %= program_usize;
            },
            Moves::Exchange(a, b) => {
                let a_adj = (a + spin_offset) % program_usize;
                let b_adj = (b + spin_offset) % program_usize;
                let a_prog = *index_to_program.get(&a_adj).unwrap();
                let b_prog = *index_to_program.get(&b_adj).unwrap();
                index_to_program.insert(a_adj, b_prog);
                index_to_program.insert(b_adj, a_prog);
            },
            _ => {}
        }
    }

    for m in move_vec.iter() {
        match *m {
            Moves::Swap(a_prog, b_prog) => {
                let a = *program_to_index.get(&a_prog).unwrap();
                let b = *program_to_index.get(&b_prog).unwrap();
                program_to_index.insert(a_prog, b);
                program_to_index.insert(b_prog, a);
            },
            _ => {}
        }
    }

    let mut orig_program_to_index: HashMap<char,usize> = HashMap::new();
    let mut orig_index_to_program: HashMap<usize,char> = HashMap::new();

    for i in 0..program_u32 {
        orig_program_to_index.insert(char::from_u32(97 + i).unwrap(), i as usize);
        orig_index_to_program.insert(i as usize, char::from_u32(97 + i).unwrap());
    }

    let mut one_index_step: HashMap<usize,usize> = HashMap::new();

    for i in 0..program_u32 {
        let adj_ind = (i as usize + spin_offset) % program_usize;
        let next_char = *index_to_program.get(&adj_ind).unwrap();
        let orig_index = *orig_program_to_index.get(&next_char).unwrap();
        one_index_step.insert(orig_index,i as usize);
    }

    let mut one_swap_step: HashMap<char,char> = HashMap::new();

    for (c, i) in program_to_index.iter() {
        let new_c = *orig_index_to_program.get(i).unwrap();
        one_swap_step.insert(*c, new_c);
    }

    let mut billion_index_steps = one_index_step.clone();
    let mut billion_swap_steps = one_swap_step.clone();

    for _ in 0..9 {
        billion_index_steps = take_index_step_ten_times(&billion_index_steps);
        billion_swap_steps = take_swap_step_ten_times(&billion_swap_steps);
    }

    let mut final_index_step = HashMap::new();
    for (k, v) in billion_index_steps.iter() {
        final_index_step.insert(v, k);
    }
    let mut final_swap_step = HashMap::new();
    for (k, v) in billion_swap_steps.iter() {
        final_swap_step.insert(v, k);
    }

    let mut result = String::from("");

    for i in 0..program_u32 {
        let u_i = i as usize;
        let ind = *final_index_step.get(&u_i).unwrap();
        let c = *orig_index_to_program.get(&ind).unwrap();
        let swap_c = *final_swap_step.get(&c).unwrap();
        result.push(*swap_c);
    }

    println!("{}", result);
}

fn take_index_step_ten_times(step: &HashMap<usize,usize>) -> HashMap<usize,usize> {
    let mut ten_steps = HashMap::new();
    for i in 0..16 {
        let mut j = i;

        for _ in 0..10 {
            j = *step.get(&j).unwrap();
        }

        ten_steps.insert(i, j);
    }

    ten_steps
}

fn take_swap_step_ten_times(step: &HashMap<char,char>) -> HashMap<char,char> {
    let mut ten_steps = HashMap::new();
    for &i in step.keys() {
        let mut j = i;

        for _ in 0..10 {
            j = *step.get(&j).unwrap();
        }

        ten_steps.insert(i, j);
    }

    ten_steps
}
