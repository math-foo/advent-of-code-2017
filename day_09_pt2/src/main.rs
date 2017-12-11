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
    let mut in_garbage = false;
    let mut ignore = false;
    let mut garbage_score = 0;

    for x in trimmed_str.chars() {
        if ignore {
            ignore = false;
        } else {
            match x {
                '!' => {
                    if in_garbage {
                        ignore = true;
                    }
                },
                '<' => {
                    if in_garbage {
                        garbage_score += 1;
                    }
                    in_garbage = true;
                },
                '>' => {
                    in_garbage = false;
                },
                _ => {
                    if in_garbage {
                        garbage_score += 1;
                    }
                }
            }
        }
    }

    println!("{}", garbage_score);
}
