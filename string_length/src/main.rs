const HELLO_WORLD : &'static str = "Halló heimur";

fn main() {
    println!("{} is {} characters long.",
        HELLO_WORLD,
        HELLO_WORLD.len()
    );
}
