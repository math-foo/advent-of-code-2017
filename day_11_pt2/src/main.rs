use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "input";
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");


    let chars_to_trim: &[char] = &[' ', '\n'];
    let trimmed_str: &str = contents.trim_matches(chars_to_trim);

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut max_steps: i32 = 0;

    for step in trimmed_str.split(',') {
        match step {
            "n" => y += 2,
            "s" => y -= 2,
            "ne" => {
                x += 1;
                y += 1;
            },
            "nw" => {
                x -= 1;
                y += 1;
            },
            "se" => {
                x += 1;
                y -= 1;
            },
            "sw" => {
                x -= 1;
                y -= 1;
            },
            _ => println!("not covered!"),
        }

        let current_steps = position_to_steps(x, y);
        if current_steps > max_steps {
            max_steps = current_steps;
        }
    }

    println!("{}", max_steps);
}

fn position_to_steps(x : i32, y: i32) -> i32 {
    let x0 = x.abs();
    let y0 = y.abs();

    if x0 <= y0 {
        let result = x0 + (y0 - x0)/2;
        result
    } else {
        x0
    }
}
