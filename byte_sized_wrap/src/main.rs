use std::num::Wrapping;

fn main() {
    let mut counter = Wrapping(0i8);
    loop {
        println!("{}", counter);
        counter += Wrapping(1i8);
    }
}