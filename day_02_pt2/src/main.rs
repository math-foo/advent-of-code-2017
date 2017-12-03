use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "input";
    println!("In file {}", filename);
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");


    let chars_to_trim: &[char] = &[' ', '\n'];
    let trimmed_str: &str = contents.trim_matches(chars_to_trim);

    let sum_of_diffs: i32 = trimmed_str.lines().map(|line| find_line_quotient(convert_to_numbers(line))).sum();
    println!("{}", sum_of_diffs);
}

fn convert_to_i32(word: &str) -> i32 {
    let a: i32 = word.parse().unwrap();
    a
}

fn convert_to_numbers(line: &str) -> Vec<i32> {
    line.split_whitespace().map(|word| convert_to_i32(word)).collect::<Vec<i32>>()
}

fn find_line_quotient(numbers: Vec<i32>) -> i32 {
    let mut q: i32 = 0;
    for a in numbers.iter() {
        for b in numbers.iter() {
            if (a % b == 0) && (a != b) {
                q = a / b;
            }
        }
    }

    q
}
