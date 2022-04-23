fn double_it(n: i32) -> i32 {
    n * 2
}

fn double_it(n: f32) -> f32 {
    n * 2.0
}

fn main() {
    println!("2 * 4 = {}", double_it(2));
}
