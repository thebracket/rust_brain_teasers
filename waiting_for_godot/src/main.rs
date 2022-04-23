async fn hello() {
    println!("Hello, World!")
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    hello();
    Ok(())
}