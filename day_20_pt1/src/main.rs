use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq,Debug,Clone)]
struct ThreeDir {
    x: i32,
    y: i32,
    z: i32,
}

impl ThreeDir {
    fn three_dir_add(&self, other: &ThreeDir) -> ThreeDir {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;

        ThreeDir {
            x,
            y,
            z
        }
    }
}

#[derive(Debug)]
struct Particle {
    id: i32,
    pos: ThreeDir,
    vel: ThreeDir,
    acc: ThreeDir,
}

impl Particle {
    fn dist_to_zero(&self) -> i32 {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }

    fn leaving_zero(&self) -> bool {
        let x_leaving_zero = (self.acc.x > 0 && self.vel.x > 0 && self.pos.x > 0) || (self.acc.x < 0 && self.vel.x < 0 && self.pos.x < 0);
        let y_leaving_zero = (self.acc.y > 0 && self.vel.y > 0 && self.pos.y > 0) || (self.acc.y < 0 && self.vel.y < 0 && self.pos.y < 0);
        let z_leaving_zero = (self.acc.z > 0 && self.vel.z > 0 && self.pos.z > 0) || (self.acc.z < 0 && self.vel.z < 0 && self.pos.z < 0);

        x_leaving_zero && y_leaving_zero && z_leaving_zero
    }

    fn next_tick_particle(&self) -> Particle {
        let new_acc = self.acc.clone();
        let new_vel = self.vel.three_dir_add(&self.acc);
        let new_pos = self.pos.three_dir_add(&new_vel);

        Particle {
            id: self.id,
            pos: new_pos,
            vel: new_vel,
            acc: new_acc
        }
    }

    fn further_from_zero(&self, other: &Particle) -> bool {
        let base_acc = self.acc.x.abs() + self.acc.y.abs() + self.acc.z.abs();
        let other_acc = other.acc.x.abs() + other.acc.y.abs() + other.acc.z.abs();


        if other_acc > base_acc {
            true
        } else if base_acc > other_acc{
            false
        } else {
            false
        }
    }
}

fn main() {
    let filename = "input";
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let mut particles: Vec<Particle> = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        let id = i as i32;
        let sections: Vec<&str> = line.split(", ").collect();
        let pos = parse_three_dir(sections[0]);
        let vel = parse_three_dir(sections[1]);
        let acc = parse_three_dir(sections[2]);

        let new_particle = Particle {
            id,
            pos,
            vel,
            acc
        };
        particles.push(new_particle);
    }

    let mut closest = &particles[0];

    for i in 1..1000 {
        let current = &particles[i];
        if !closest.further_from_zero(current) {
            closest = current;
        }
    }

    println!("{}", closest.id);
}

fn parse_three_dir(base_str: &str) -> ThreeDir {
    let position_str = base_str.split('<').nth(1).unwrap().split('>').nth(0).unwrap();
    let position_vec: Vec<&str> = position_str.split(',').collect();
    let position_values: Vec<i32> = position_vec.iter().map(|a| a.parse::<i32>().unwrap()).collect();
    let position = ThreeDir {
        x: position_values[0],
        y: position_values[1],
        z: position_values[2]
    };
    position
}
