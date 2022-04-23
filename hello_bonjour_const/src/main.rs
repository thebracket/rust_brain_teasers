enum Language { English, French }

const fn hello(language: Language) -> &'static str {
    match language {
        Language::English => "Hello World",
        Language::French => "Bonjour le monde",
    }
}

fn main() {
    println!("{}", hello(Language::English));
}
