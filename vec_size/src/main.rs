fn main() {
    let mut my_vec = Vec::with_capacity(1);
    my_vec.push("Hello");
    println!("{}", my_vec.capacity());
    my_vec.push("World");
    println!("{}", my_vec.capacity());
}
