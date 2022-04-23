fn main() {
    #[cfg(feature = "english")]
    let hello = || println!("Hello World");
    #[cfg(feature = "french")]
    let hello = || println!("Bonjour le monde");
    hello();
}
