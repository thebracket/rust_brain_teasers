fn double_it(n: u64, _: i32) -> u64 {
    n * 2
}

fn main() {
    let one: i32 = 1;
    let n = double_it(one as _, 3);
    println!("{}", n);
}
