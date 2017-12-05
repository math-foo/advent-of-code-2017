use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let filename = "input"; 
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let mut valid_passphrases = 0;

    for line in contents.lines() {
        let total_word_count = line.split_whitespace().count();
        let mut unique_words = HashSet::new();
        for word in line.split_whitespace() {
            unique_words.insert(sort(word));
        }
        let unique_word_count = unique_words.len();
        if unique_word_count == total_word_count {
            valid_passphrases += 1;
        }
    }

    println!("{}", valid_passphrases);
}

fn sort(word: &str) -> String {
    let mut temp = word.chars().collect::<Vec<char>>();
    temp.sort();
    temp.into_iter().collect::<String>()
}
