use crate::model::{Area, Color, Coord, Image};
// LIFE
pub const HIGH_LIFE_POS: &Coord = &Coord { x: 860, y: 48 };
pub const MID_LIFE_POS: &Coord = &Coord { x: 680, y: 48 };
pub const LOW_LIFE_POS: &Coord = &Coord { x: 480, y: 48 };
pub const LIFE_COLOR: &Color = &Color { r: 0, g: 50, b: 0 };

// MANA
pub const HIGH_MANA_POS: &Coord = &Coord { x: 1050, y: 40 };
pub const MID_MANA_POS: &Coord = &Coord { x: 1360, y: 40 };
pub const LOW_MANA_POS: &Coord = &Coord { x: 1450, y: 40 };
pub const MANA_COLOR: &Color = &Color { r: 0, g: 0, b: 100 };

//MAP
pub const MAP_AREA: &Area = &Area {
	top_left: Coord { x: 1753, y: 38 },
	bot_right: Coord { x: 1857, y: 144 },
};

// MAP_Waypoints

// OUTSIDE UI
pub const ICON_SELECTED: &Coord = &Coord { x: 222, y: 1070 };
pub const ICON_SELECTED_COLOR: &Color = &Color {
	r: 81,
	g: 81,
	b: 81,
};
pub const LOGIN_SCREEN: &Coord = &Coord { x: 582, y: 40 };
pub const LOGIN_SCREEN_COLOR: &Color = &Color {
	r: 255,
	g: 255,
	b: 255,
};
pub const WINDOW_EXIST: &Coord = &Coord { x: 230, y: 1078 };
pub const WINDOW_EXIST_COLOR: &Color = &Color {
	r: 31,
	g: 158,
	b: 222,
};

pub const WAITING_EMAIL: &Coord = &Coord { x: 846, y: 481 };
pub const WAITING_EMAIL_COLOR: &Color = &Color {
	r: 144,
	g: 144,
	b: 144,
};

pub const LOGIN_BUTTON: &Coord = &Coord { x: 1010, y: 597 };
pub const BRAVE_BUTTON: &Coord = &Coord { x: 114, y: 1062 };
pub const EMAIL_LINK: &Coord = &Coord { x: 700, y: 91 };
pub const FIRST_EMAIL: &Coord = &Coord { x: 400, y: 381 };
pub const EMAIL_CODE: &Coord = &Coord { x: 820, y: 693 };
pub const CODE_BOX: &Coord = &Coord { x: 823, y: 538 };
pub const DELETE_EMAIL: &Coord = &Coord { x: 424, y: 201 };
pub const SEND_CODE: &Coord = &Coord { x: 1051, y: 580 };
pub const CHAR_SELECT: &Coord = &Coord { x: 1235, y: 718 };

pub fn get_vec_position(bounds: &Coord, position: &Coord) -> Result<usize, &'static str> {
	if position.x > bounds.x || position.y > bounds.y {
		return Err("Wrong size");
	}
	Ok((position.y * bounds.x + position.x) as usize)
}

pub fn get_color_at_position(image: &Image, position: &Coord, isprint: bool) -> Color {
	let color = image.pixels[get_vec_position(image.bounds, position).unwrap()];
	if isprint {
		println!("{} {} {}", color.r, color.g, color.b)
	}

	Color {
		r: color.r,
		g: color.g,
		b: color.b,
	}
}

pub fn color_greater_than(first: &Color, second: &Color) -> bool {
	let r = first.r >= second.r;
	let g = first.g >= second.g;
	let b = first.b >= second.b;
	r && g && b
}

pub fn has_greater_color_at_position(image: &Image, position: &Coord, color: &Color) -> bool {
	color_greater_than(&get_color_at_position(image, position, false), color)
}

pub fn has_color_at_position(
	image: &Image,
	position: &Coord,
	color: &Color,
	isprint: bool,
) -> bool {
	&get_color_at_position(image, position, isprint) == color
}

pub fn get_color_positions_in_area(image: &Image, area: &Area, color: &Color) -> Vec<Coord> {
	let mut result = Vec::new();
	for x in area.top_left.x..area.bot_right.x {
		for y in area.top_left.y..area.bot_right.y {
			let coord = Coord { x, y };
			if has_color_at_position(image, &coord, color, false) {
				result.push(coord);
			}
		}
	}
	result
}

pub fn new_coord(ponto: &(u32, u32)) -> Coord {
	Coord {
		x: ponto.0,
		y: ponto.1,
	}
}
