use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

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
    fn posn_str(&self) -> String {
        format!("{}_{}_{}", self.pos.x, self.pos.y, self.pos.z)
    }

    fn tick(&self) -> Particle {
        let new_acc = self.acc.clone();
        let new_vel = self.vel.three_dir_add(&new_acc);
        let new_pos = self.pos.three_dir_add(&new_vel);

        Particle {
            id: self.id,
            pos: new_pos,
            vel: new_vel,
            acc: new_acc
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

  
    for i in 1..1000 {
        let mut seen_positions = HashSet::new();
        let mut to_destroy = HashSet::new();
        for particle in particles.iter() {
            let particle_pos_str = particle.posn_str();
            if seen_positions.contains(&particle_pos_str) {
                to_destroy.insert(particle_pos_str.clone());
            }
            seen_positions.insert(particle_pos_str.clone());
        }

        let mut index_to_destroy = particles.iter().position(|x| to_destroy.contains(&x.posn_str()));

        while index_to_destroy != None {
            particles.swap_remove(index_to_destroy.unwrap());
            index_to_destroy = particles.iter().position(|x| to_destroy.contains(&x.posn_str()));
        }

        let mut new_particles = Vec::new();

        for particle in particles.iter() {
            new_particles.push(particle.tick());
        }

        particles = new_particles;
    }

    println!("{}", particles.len());
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
