use std::thread;
use std::time::Duration;

fn count_and_wait(n: u64) -> u64 {
    println!("Starting {}", n);
    std::thread::sleep(Duration::from_millis(n * 100));
    println!("Returning {}", n);
    n
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = thread::spawn(|| count_and_wait(1));
    let b = thread::spawn(|| count_and_wait(2));
    let c = thread::spawn(|| count_and_wait(3));
    a.join().unwrap();
    b.join().unwrap();
    c.join().unwrap();
    Ok(())
}
