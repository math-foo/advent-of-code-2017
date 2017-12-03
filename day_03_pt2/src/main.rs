fn main() {
    let input = 265149;
    let mut computed_values: Vec<Vec<i32>> = Vec::new();
    let bottom_layer: Vec<i32> = vec![1];
    computed_values.push(bottom_layer);
    let first_layer: Vec<i32> = vec![1,2,4,5,10,11,23,25];
    computed_values.push(first_layer);

    let mut layer = 1;
    let mut max_at_layer = 25;

    while input > max_at_layer {
        layer = layer + 1;
        let new_layer = add_layer(layer, &computed_values);
        max_at_layer = *new_layer.last().unwrap();
        computed_values.push(new_layer);
    }

    for x in computed_values.last().unwrap().iter() {
        if *x > input {
            println!("{}", x);
            break;
        }
    }
}

fn add_layer(layer: i32, computed_values: &Vec<Vec<i32>>) -> Vec<i32> {
    let elements_at_layer = 8 * layer;
    let mut layer_values : Vec<i32> = Vec::new();
    let previous_layer = computed_values.last().unwrap();
    let mut layer_offset = 0;
    let mut step = 0;
    let before_corner = |i: i32| -> bool {i == (2 * layer) - 2 || i == (4 * layer) - 2 || i == (6 * layer) - 2};
    let is_corner = |i: i32| -> bool {i == (2 * layer) - 1 || i == (4 * layer) - 1 || i == (6 * layer) - 1};
    let after_corner = |i: i32| -> bool {i == (2 * layer) || i == (4 * layer) || i == (6 * layer)};
    let before_last_corner = |i: i32| -> bool {i == 8 * layer - 2};
    let last_corner = |i: i32| -> bool {i == 8 * layer - 1};

    while step < elements_at_layer {
        match step {
            0 => {
                layer_values.push(previous_layer.last().unwrap() + previous_layer.first().unwrap());
            }
            1 => {
                let mut value: i32 = *layer_values.last().unwrap();
                value += previous_layer.last().unwrap() + previous_layer.first().unwrap();
                value += previous_layer[1];
                layer_values.push(value);
            }
            i if before_corner(i) => {
                let mut value: i32 = *layer_values.last().unwrap();
                value += previous_layer[layer_offset] + previous_layer[layer_offset + 1];
                layer_offset += 1;
                layer_values.push(value);
            }
            i if is_corner(i) => {
                let mut value: i32 = *layer_values.last().unwrap();
                value += previous_layer[layer_offset];
                layer_values.push(value);
            }
            i if after_corner(i) => {
                let mut value: i32 = *layer_values.last().unwrap() * 2;
                value += previous_layer[layer_offset + 1];
                layer_values.push(value);
            }
            i if before_last_corner(i) => {
                let mut value: i32 = *layer_values.last().unwrap() + *layer_values.first().unwrap();
                value += previous_layer[layer_offset] + previous_layer[layer_offset + 1];
                layer_offset += 1;
                layer_values.push(value);
            }
            i if last_corner(i) => {
                let mut value: i32 = *layer_values.last().unwrap() + *layer_values.first().unwrap();
                value += previous_layer[layer_offset];
                layer_values.push(value);
            }
            _ => {
                let mut value: i32 = *layer_values.last().unwrap();
                value += previous_layer[layer_offset] + previous_layer[layer_offset + 1] + previous_layer[layer_offset + 2];
                layer_offset += 1;
                layer_values.push(value);
            }
        }
        step += 1;
    }
    layer_values
}
