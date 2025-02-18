use captrs::Bgr8;

#[derive(Debug)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

pub struct Area {
    pub top_left: Coord,
    pub bot_right: Coord,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
    pub is_attacking: bool,
    pub is_moving: bool,
    pub has_cap: bool,
    pub life: u8,
    pub mana: u8,
    pub food_timer: i64,
    pub move_timer: i64,
    pub next_waypoint: usize,
    pub ladder_cooldown: i64,
}
