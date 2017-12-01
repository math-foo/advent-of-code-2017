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

    // first prev is last value since strings are circular
    let mut prev = trimmed_str.chars().last().unwrap().to_digit(RADIX).unwrap();
    let mut sum = 0;

    for i in trimmed_str.chars().map(|c| c.to_digit(RADIX).unwrap()) {
      if i == prev {
          sum = sum + i;
      }

      prev = i;
    }

    println!("sum: {}", sum);
}
