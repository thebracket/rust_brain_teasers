use tokio::join;
use std::time::Duration;

async fn count_and_wait(n: u64) -> u64 {
    println!("Starting {}", n);
    std::thread::sleep(Duration::from_millis(n * 100));
    println!("Returning {}", n);
    n
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Join runs multiple tasks concurrently and returns when they all
    // complete execution.
    join!(count_and_wait(1), count_and_wait(2), count_and_wait(3));
    Ok(())
}
