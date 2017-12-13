use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let filename = "input";
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");


    let chars_to_trim: &[char] = &[',',' ', '\n'];
    let trimmed_str: &str = contents.trim_matches(chars_to_trim);

    let mut pipes: Vec<HashSet<u32>> = Vec::new();

    for _ in 0..2000 {
        pipes.push(HashSet::new());
    }

    for entry in trimmed_str.lines() {
        let index: u32 = entry.split_whitespace().nth(0).unwrap().parse().unwrap();
        for raw in entry.split(">").last().unwrap().split_whitespace() {
            let number: u32 = raw.trim_matches(chars_to_trim).parse().unwrap();

            pipes[index as usize].insert(number);
            pipes[number as usize].insert(index);
        }
    }

    let mut have_checked: HashSet<u32> = HashSet::new();
    let mut to_check: HashSet<u32> = HashSet::new();

    for n in &pipes[0] {
        if *n != 0 {
            to_check.insert(*n);
        }
    }

    have_checked.insert(0);

    while !to_check.is_empty() {
        let n = *to_check.iter().next().unwrap();
        to_check.remove(&n);
        have_checked.insert(n);
        for m in &pipes[n as usize] {
            if !have_checked.contains(m) {
                to_check.insert(*m);
            }
        }
    }

    println!("{}", have_checked.len());
}

