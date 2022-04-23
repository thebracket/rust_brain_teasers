fn double_it<T>(n: T) -> T //(1)
where T: std::ops::Mul<Output = T> + From<i32>//(2)
{
    n * 2.into()//(3)
}

fn main() {
    println!("2+2 = {}", double_it(2));
}
