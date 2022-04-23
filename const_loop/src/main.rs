const fn fib(n: u128) -> u128 {
    let mut a = 1;
    let mut b = 1;
    for _ in 2..n {
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    b
}

fn main() {
    for i in 0..5 {
        println!("Fib {} = {}", i, fib(i));
    }
}
