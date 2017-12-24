
// This is a translated version of my input assembly, do not expect it to work for your input
fn main() {
    let mut d: i64 = 0;
    let mut b: i64 = (99 * 100) + 100000;
    let c: i64 = b + 17000;
    let mut h: i64 = 0;

    while b <= c {
        let mut increment_h = false;
        d = 2;

        while d != b {
            if b % d == 0 {
                increment_h = true;
            }

            d += 1;
        }

        if increment_h {
            h += 1;
        }
        b += 17;
    }

    println!("{}", h);
}

