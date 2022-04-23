#[derive(Debug)]
struct Parser<'a> {
    body: String,
    subtext : &'a str,
}

fn main() {
    let mut document = Parser {
        body: "Hello".to_string(),
        subtext: ""
    };
    document.subtext = &document.body;

    let b = document;
    println!("{:?}", b);
}
