use std::collections::HashSet;

fn main() {
    let input_str = "ugkiagan";
    let mut region_map = HashSet::new();

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

      for (col, entry) in dense_vec.iter().enumerate() {
          let actual_col = (col * 8) as u32;
          map_digits(*entry, row, actual_col, &mut region_map)
      }
    }

    let mut region_count = 0;

    for row in 0..128 {
        for col in 0..128 {
            let index = col * 1000 + row;

            if region_map.contains(&index) {
                remove_region(row, col, &mut region_map);
                region_count += 1;
            }
        }
    }

    println!("{}", region_count);
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

fn map_digits(x: u8, row: u32, col: u32, region_map: &mut HashSet<u32>) {
    let mut col_a = col + 8;
    let mut b = x;

    while b != 0 {
        col_a -= 1;
        if b % 2 == 1 {
            let index = (col_a as u32) * 1000 + row;
            region_map.insert(index);
        }

        b /= 2;
    }
}

fn remove_region(row: u32, col: u32, region_map: &mut HashSet<u32>) {
    let index = col * 1000 + row;

    if region_map.contains(&index) {
        region_map.remove(&index);
        
        if row > 0 {
            remove_region(row - 1, col, region_map);
        }
        
        if row < 127 {
            remove_region(row + 1, col, region_map);
        }
        
        if col > 0 {
            remove_region(row, col - 1, region_map);
        }
        
        if col < 127 {
            remove_region(row, col + 1, region_map);
        }
    }
}
