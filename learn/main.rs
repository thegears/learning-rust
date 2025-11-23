use std::io;

use rand::random;

fn main() {
    println!("qwwwqweqweww");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    println!("{input} {}", get_random_number(3))
}

fn get_random_number(time: u32) -> f32 {
    random::<f32>() * (time as f32)

    //We are using the top one instead of this line, we can use either one but the top one is cleaner
    // return random::<f32>();
}
