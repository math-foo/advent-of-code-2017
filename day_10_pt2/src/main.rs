use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "input";
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");


    let chars_to_trim: &[char] = &[' ', '\n'];
    let trimmed_str: &str = contents.trim_matches(chars_to_trim);
    let mut ascii_values = Vec::new();
    for i in trimmed_str.as_bytes() {
        ascii_values.push(*i as u32);
    }

    // hard coded values
    ascii_values.push(17);
    ascii_values.push(31);
    ascii_values.push(73);
    ascii_values.push(47);
    ascii_values.push(23);

    let mut knot: Vec<u32> = (0..256).collect();

    let mut cur_pos = 0;
    let mut cur_skip = 0;

    for _ in 0..64 {
        for i in ascii_values.iter() {
            let first = cur_pos as usize;
            let last = ((cur_pos + i - 1) % 256) as usize;
            twist(&mut knot, first, last);
            cur_pos = (cur_pos + i + cur_skip) % 256;
            cur_skip += 1;
        }
    }

    let mut dense_vec = Vec::new();

    for part in knot.chunks(16) {
        let mut xored_part = 0;
        for x in part.iter() {
            xored_part ^= *x;
        }

        dense_vec.push(xored_part as u8);
    }

    let result = to_hex_string(dense_vec);
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

fn to_hex_string(bytes: Vec<u8>) -> String {
    let hex_strs: Vec<String> = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    hex_strs.join("")
}
