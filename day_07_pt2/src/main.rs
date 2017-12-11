use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let filename = "input";
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");


    let chars_to_trim: &[char] = &[' ', '\n', ','];
    let trimmed_str: &str = contents.trim_matches(chars_to_trim);
    let mut programs = HashMap::new();
    let mut supported_programs = HashMap::new();

    for entry in trimmed_str.lines() {
        parse_entry(entry, &mut programs, &mut supported_programs);
    }

    let mut all_supported_programs = HashSet::new();

    for program_vector in supported_programs.values() {
        for program in program_vector {
            all_supported_programs.insert(program);
        }
    }

    // first unblanced program must be the base
    let mut invalid_program = String::from("");

    for program in programs.keys() {
        if !all_supported_programs.contains(program) {
            invalid_program = program.to_string();
        }
    }

    let mut final_weight = HashMap::new();

    for (program, weight) in programs.iter() {
        if !supported_programs.contains_key(program) {
            final_weight.insert(program, *weight);
        }
    }

    while final_weight.keys().count() != programs.keys().count() {
        for (program, weight) in programs.iter() {
            if supported_programs.contains_key(program) {
                let carried_programs = &supported_programs[program];
                let mut all_known = true;
                let mut new_weight = *weight;
                for carried_program in carried_programs.iter() {
                    if !final_weight.contains_key(carried_program) {
                        all_known = false;
                    } else {
                        new_weight += final_weight[carried_program];
                    }
                }

                if all_known {
                    final_weight.insert(program, new_weight);
                }
            }
        }
    }

    let mut first_layer = HashMap::new();
    for current_program in supported_programs[&invalid_program].iter() {
        let weight = final_weight[current_program];
        first_layer.insert(current_program, weight);
    }

    let low_value = first_layer.values().min().unwrap();
    let high_value = first_layer.values().max().unwrap();
    let off_by = high_value - low_value; // by inspection

    let mut found_bad_program = false;

    while !found_bad_program {
      let mut current_layer = HashSet::new();
      for current_program in supported_programs[&invalid_program].iter() {
          let weight = final_weight[current_program];
          current_layer.insert(weight);
      }
      let low_value = *current_layer.iter().min().unwrap();
      let high_value = *current_layer.iter().max().unwrap();

      // this is it!
      if low_value == high_value {
          let correct_weight = programs[&invalid_program] - off_by;
          println!("{}", correct_weight);
          found_bad_program = true;
      } else {
          for current_program in supported_programs[&invalid_program].iter() {
              if high_value == final_weight[current_program] {
                  invalid_program = current_program.to_string();
              }
          }
      }
    }

}

fn parse_entry(entry: &str, programs: &mut HashMap<String, u32>, supported_programs: &mut HashMap<String, Vec<String>>) {
    let chars_to_trim: &[char] = &['(',')',','];
    let name: String = entry.split_whitespace().nth(0).unwrap().to_string();
    let weight: u32 = entry.split_whitespace().nth(1).unwrap().trim_matches(chars_to_trim).parse().unwrap();

    programs.insert(name, weight);

    let count = entry.split_whitespace().count();

    if count > 2 {
        let again_name: String = entry.split_whitespace().nth(0).unwrap().to_string();
        let mut program_vector: Vec<String> = Vec::new();
        let mut index = 3;

        while index < count {
            let supported_name: String = entry.split_whitespace().nth(index).unwrap().trim_matches(chars_to_trim).to_string();
            program_vector.push(supported_name);
            index += 1;
        }

        supported_programs.insert(again_name, program_vector);
    }
}

