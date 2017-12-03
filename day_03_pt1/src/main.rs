fn main() {
    let input = 265149;

    let mut layer = 0;
    let mut prev_max_at_layer = -1;
    let mut max_at_layer = 1;

    while input > max_at_layer {
        layer = layer + 1;
        prev_max_at_layer = max_at_layer;
        max_at_layer = max_at_layer + 4 + 4 * (2 * layer - 1);
    }

    let mut dir = -1;
    let mut dist = 2 * layer -1;
    let mut step = prev_max_at_layer + 1;

    while input > step {
        step = step + 1;
        dist = dist + dir;
        if dist == layer {
            dir = 1;
        }
        if dist == 2 * layer {
            dir = -1;
        }
    }

    println!("{}", dist);
}
