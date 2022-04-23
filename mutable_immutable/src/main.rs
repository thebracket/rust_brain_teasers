fn main() {
    let life_the_universe = &mut 41;
    *life_the_universe += 1;
    println!("Life, the Universe and Everything: {}", life_the_universe);
}
