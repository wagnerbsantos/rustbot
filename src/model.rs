use captrs::Bgr8;

pub struct Coord {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Image<'a> {
    pub bounds: &'a Coord,
    pub pixels: &'a [Bgr8],
}

#[derive(Debug, Copy, Clone)]
pub struct Status {
    pub life: u8,
    pub mana: u8,
    pub food_timer: i64,
}
