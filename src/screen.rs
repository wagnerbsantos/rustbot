use crate::model::{Color, Coord, Image};
// LIFE
pub const HIGH_LIFE_POS: &Coord = &Coord { x: 850, y: 40 };
pub const MID_LIFE_POS: &Coord = &Coord { x: 580, y: 40 };
pub const LOW_LIFE_POS: &Coord = &Coord { x: 480, y: 40 };
pub const LIFE_COLOR: &Color = &Color { r: 0, g: 50, b: 0 };

// MANA
pub const HIGH_MANA_POS: &Coord = &Coord { x: 1100, y: 40 };
pub const MID_MANA_POS: &Coord = &Coord { x: 1330, y: 40 };
pub const LOW_MANA_POS: &Coord = &Coord { x: 1450, y: 40 };
pub const MANA_COLOR: &Color = &Color {
    r: 0,
    g: 57,
    b: 126,
};

// OUTSIDE UI
pub const ICON_SELECTED: &Coord = &Coord { x: 340, y: 1079 };
pub const ICON_SELECTED_COLOR: &Color = &Color {
    r: 21,
    g: 101,
    b: 192,
};
pub const LOGIN_SCREEN: &Coord = &Coord { x: 526, y: 171 };
pub const LOGIN_SCREEN_COLOR: &Color = &Color {
    r: 240,
    g: 159,
    b: 161,
};

pub fn get_vec_position(bounds: &Coord, position: &Coord) -> Result<usize, &'static str> {
    if position.x > bounds.x || position.y > bounds.y {
        return Err("Wrong size");
    }
    return Ok((position.y * bounds.x + position.x) as usize);
}

pub fn get_color_at_position(image: &Image, position: &Coord) -> Color {
    let color = image.pixels[get_vec_position(image.bounds, position).unwrap()];
    return Color {
        r: color.r,
        g: color.g,
        b: color.b,
    };
}

pub fn color_greater_than(first: &Color, second: &Color) -> bool {
    let r = first.r >= second.r;
    let g = first.g >= second.g;
    let b = first.b >= second.b;
    return r && g && b;
}

pub fn has_greater_color_at_position(image: &Image, position: &Coord, color: &Color) -> bool {
    return color_greater_than(&get_color_at_position(image, position), color);
}

pub fn has_color_at_position(image: &Image, position: &Coord, color: &Color) -> bool {
    return &get_color_at_position(image, position) == color;
}

pub fn new_coord(ponto: &(u32, u32)) -> Coord {
    return Coord {
        x: ponto.0,
        y: ponto.1,
    };
}
