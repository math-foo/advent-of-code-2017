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

    const RADIX: u32 = 10;
    let length = trimmed_str.chars().count();
    let offset = length / 2 ;
    let digit_vec:Vec<char> = trimmed_str.chars().collect();
    let mut sum = 0;

    // first prev is last value since strings are circular
//    let mut prev = trimmed_str.chars().last().unwrap().to_digit(RADIX).unwrap();
    for i in 0..offset {
        let a = digit_vec[i].to_digit(RADIX).unwrap();
        let b = digit_vec[i + offset].to_digit(RADIX).unwrap();

        if a == b {
            sum = sum + a + b
        }
    }


    println!("sum: {}", sum);
}
