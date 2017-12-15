use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "input";
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");


    let chars_to_trim: &[char] = &[',',' ', '\n'];
    let trimmed_str: &str = contents.trim_matches(chars_to_trim);

    let mut severity = 0;

    for entry in trimmed_str.lines() {
        let depth: u32 = entry.split(": ").nth(0).unwrap().parse().unwrap();
        let range: u32 = entry.split(": ").last().unwrap().parse().unwrap();

        if depth != 0 {
            let freq = 2 * range - 2;
            if freq == 0 {
                severity += depth * range;
            } else {
                let detected = (depth % freq) == 0;
                if detected {
                    severity += depth * range;
                }
            }
        }
    }

    println!("{}", severity);
}

