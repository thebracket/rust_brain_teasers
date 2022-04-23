fn main() {
    let x : u64 = 4_294_967_296;
    let y = x as u32;
    if x  == y as u64 {
        println!("x equals y.");
    } else {
        println!("x does not equal y.");
    }
}
