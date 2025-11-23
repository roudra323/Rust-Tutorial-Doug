fn add(num_a: f32, num_b: f32) -> f32 {
    if num_a == 0.0 || num_b == 0.0 {
        0.0
    } else {
        num_a + num_b
    }
}

fn say_hi() -> () {
    println!("I am saying Hi");
}

fn main() {
    let sum = add(1.0, 2.0);
    println!("{}", sum);

    say_hi();
}
