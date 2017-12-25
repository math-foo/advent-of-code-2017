use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

#[derive(Debug)]
struct Domino {
    id: i32,
    a: i32,
    b: i32
}

impl Domino {
    fn matches(&self, x: i32) -> bool {
      return x == self.a || x == self.b
    }

    fn strength(&self) -> i32 {
      return self.a + self.b
    }
}

fn main() {
    let filename = "input";
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let mut id = 0;
    let mut dominos: Vec<Domino> = Vec::new();

    for line in contents.lines() {
      let a: i32 = line.split('/').nth(0).unwrap().parse().unwrap();
      let b: i32 = line.split('/').nth(1).unwrap().parse().unwrap();

      let new_domino = Domino {
        id,
        a,
        b
      };
      dominos.push(new_domino);
      id += 1;
    }

    let result = find_largest(0, &dominos, HashSet::new());
    println!("{}", result);
}

fn find_largest(last: i32, dominos: &Vec<Domino>, used: HashSet<i32>) -> i32 {
    let mut best_result = 0;

    for domino in dominos.iter() {
      if domino.matches(last) && !used.contains(&domino.id) {
        let mut new_used = used.clone();
        new_used.insert(domino.id);

        let mut new_last = 0;

        if domino.a == last {
          new_last = domino.b;          
        } else {
          new_last = domino.a;          
        }

        let new_result = find_largest(new_last, dominos, new_used) + domino.strength();
        if new_result > best_result {
          best_result = new_result;
        }
      }
    }

    best_result
}
