use std::f32::consts::PI;

pub struct Degrees(pub f32);
pub struct Radians(pub f32);

impl Degrees {
    pub fn new(angle: f32) -> Self {
        Self(angle)
    }
}

impl From<Degrees> for Radians {
    fn from(item : Degrees) -> Self {
        Self(item.0 * PI / 180.0)
    }
}

fn main() {
    let one_eighty_degrees = Degrees::new(180.0);
    let one_eighty_radians : Radians = one_eighty_degrees.into();
    println!("180 Degrees in Radians = {}", one_eighty_radians.0);
}