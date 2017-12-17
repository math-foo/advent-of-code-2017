use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "input";
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");


    let chars_to_trim: &[char] = &[' ', '\n'];
    let trimmed_str: &str = contents.trim_matches(chars_to_trim);

    let start_a: u32 = trimmed_str.lines().nth(0).unwrap().split_whitespace().last().unwrap().parse().unwrap();
    let start_b: u32 = trimmed_str.lines().last().unwrap().split_whitespace().last().unwrap().parse().unwrap();

    let factor_a = 16807;
    let factor_b = 48271;

    let a_reduce = 4;
    let b_reduce = 8;

    let judge_reduce_value = 65536;

    let mut cur_a = start_a;
    let mut cur_b = start_b;

    let mut a_values = Vec::new();
    let mut b_values = Vec::new();

    for _ in 0..5000000 {
        cur_a = next_value(cur_a, factor_a);
        cur_b = next_value(cur_b, factor_b);

        if cur_a % a_reduce == 0 {
            a_values.push(cur_a % judge_reduce_value);
        }

        if cur_b % b_reduce == 0 {
            b_values.push(cur_b % judge_reduce_value);
        }
    }

    let mut match_count = 0;

    for (a_val, b_val) in a_values.iter().zip(b_values.iter()) {
        if a_val == b_val {
            match_count += 1;
        }
    }

    println!("{}", match_count);
}

fn next_value(prev_value: u32, factor: u32) -> u32 {
    let reduce_value: u64 = 2147483647;
    let product: u64 = (prev_value as u64) * (factor as u64);
    let result = (product % reduce_value) as u32;

    result
}
