fn main() {
    let steps = 380;
    let mut memory = Vec::new();

    memory.push(0);

    let mut index = 0;

    for i in 1..50000000 {
        let circular_length = i;
        let new_index = ((index + steps) % circular_length) + 1;

        if new_index == 0 {
            println!("uh-oh");
        }

        if new_index == 1 {
            memory.insert(new_index, i);
        }
        index = new_index;
    }

    let result = memory.get((1)).unwrap();

    println!("{}", result);
}
