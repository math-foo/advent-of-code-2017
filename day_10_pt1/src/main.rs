use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "input";
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");


    let chars_to_trim: &[char] = &[' ', '\n'];
    let trimmed_str: &str = contents.trim_matches(chars_to_trim);

    let mut knot: Vec<u32> = (0..256).collect();

    let mut cur_pos = 0;
    let mut cur_skip = 0;


    for i in trimmed_str.split(',').map(|x| x.parse::<u32>().unwrap()) {
        if i > 0 {
          let first = cur_pos as usize;
          let last = ((cur_pos + i - 1) % 256) as usize;
          twist(&mut knot, first, last);
        }

        cur_pos = (cur_pos + i + cur_skip) % 256;
        cur_skip += 1;
    }

    let result = knot[0] * knot[1];
    println!("{}", result);
}

fn twist(knot: &mut Vec<u32>, first: usize, last: usize) {
    if first != last {
        let temp = knot[first];
        knot[first] = knot[last];
        knot[last] = temp;

        if first + 1 != last && (first != 255 || last != 0) {
            let new_first = (first + 1) % 256;
            let new_last = (last + 255) % 256;
            twist(knot, new_first, new_last);
        }
    }
}
