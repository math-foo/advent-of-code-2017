fn main() {
    let steps = 380;
    let mut memory = Vec::new();

    memory.push(0);

    let mut index = 0;

    for i in 1..2018 {
        let circular_length = memory.iter().len();
        let new_index = ((index + steps) % circular_length) + 1;
        memory.insert(new_index, i);
        index = new_index;
    }

    let circular_length = memory.iter().len();
    let result = memory.get((index+1) % circular_length).unwrap();

    println!("{}", result);
}
