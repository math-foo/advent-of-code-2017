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

#[derive(Debug,Clone)]
struct Bridge {
    strength: i32,
    domino_count: i32
}

impl Bridge {
    fn add_domino(&mut self, d: &Domino) {
      self.strength += d.strength();
      self.domino_count += 1;
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

    let result = find_longest(0, &dominos, HashSet::new());
    println!("{}", result.strength);
}

fn find_longest(last: i32, dominos: &Vec<Domino>, used: HashSet<i32>) -> Bridge {
    let mut best_result = Bridge {
      strength: 0,
      domino_count: 0
    };

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

        let mut new_bridge = find_longest(new_last, dominos, new_used);
        new_bridge.add_domino(domino);

        if new_bridge.domino_count > best_result.domino_count {
          best_result = new_bridge;
        } else if new_bridge.domino_count == best_result.domino_count && new_bridge.strength > best_result.strength {
          best_result = new_bridge;
        }
      }
    }

    best_result
}
