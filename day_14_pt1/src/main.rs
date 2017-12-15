fn main() {
    let input_str = "ugkiagan";
    let mut total_used = 0;

    for row in 0..128 {
      let mut ascii_values = Vec::new();
      for i in input_str.as_bytes() {
          ascii_values.push(*i as u32);
      }

      let suffix = format!("-{}", row);
      for i in suffix.as_bytes() {
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

      for entry in dense_vec.iter() {
          total_used += count_digits(*entry)
      }
    }

    println!("{}", total_used);
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

fn count_digits(x: u8) -> u32 {
    let mut a = 0;
    let mut b = x;
    while b != 0 {
        if b % 2 == 1 {
            a += 1
        }

        b /= 2
    }

    a
}
