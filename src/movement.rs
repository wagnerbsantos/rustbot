use crate::model::{Color, Coord, Image, Status};
use crate::screen::{get_color_positions_in_area, MAP_AREA};
use enigo::{Enigo, MouseButton, MouseControllable};
use shuteye::sleep;
use std::time::Duration;

#[derive()]
struct Waypoints;

impl Waypoints {
    pub const WAYPOINT_CHECK_MARK: Color = Color {
        r: 68,
        g: 206,
        b: 87,
    };
    pub const WAYPOINT_QUESTIONMARK: Color = Color {
        r: 15,
        g: 15,
        b: 255,
    };
    pub const WAYPOINT_BANNER: Color = Color { r: 1, g: 1, b: 240 };
    pub const WAYPOINT_STAR: Color = Color {
        r: 247,
        g: 148,
        b: 28,
    };
    pub const WAYPOINT_EXCLAMATION: Color = Color { r: 0, g: 0, b: 160 };
    pub const WAYPOINT_LOCK: Color = Color {
        r: 186,
        g: 141,
        b: 13,
    };
    pub const WAYPOINT_MONEY: Color = Color {
        r: 8,
        g: 110,
        b: 51,
    };
    pub const WAYPOINT_MASK: Color = Color {
        r: 71,
        g: 67,
        b: 67,
    };
}
impl Waypoints {
    const VALUES: [Color; 7] = [
        Self::WAYPOINT_STAR,
        Self::WAYPOINT_CHECK_MARK,
        Self::WAYPOINT_BANNER,
        Self::WAYPOINT_LOCK,
        Self::WAYPOINT_QUESTIONMARK,
        Self::WAYPOINT_MASK,
        Self::WAYPOINT_MONEY,
    ];
}

pub const MAP_CENTER: Coord = Coord { x: 1805, y: 92 };

pub fn use_movement(image: &Image, status: &mut Status) {
    let coords = get_color_positions_in_area(
        image,
        MAP_AREA,
        Waypoints::VALUES.get(status.next_waypoint).unwrap(),
    );
    if coords.len() > 0 {
        let next = coords.get(0).unwrap();
        let under_checkpoint = next.x >= 1802 && next.x <= 1810 && next.y >= 88 && next.y <= 96;

        if under_checkpoint && !status.is_attacking {
            status.next_waypoint = (status.next_waypoint + 1) % (Waypoints::VALUES.len() - 1);
            move_to(next);
            status.move_timer = 3;
            status.is_moving = true;
            println!("under checkpoint");
        }
        if status.move_timer <= 0 || !status.is_moving {
            if status.ladder_cooldown <= 0 && status.next_waypoint == 1 {
                right_click(&Coord { x: 950, y: 480 });
                status.ladder_cooldown = 4;
                sleep(Duration::from_millis(1000));
            } else {
                move_to(next);
                status.move_timer = 13;
                status.is_moving = true;
            }
        }
    } else {
        status.next_waypoint = (status.next_waypoint + 1) % (Waypoints::VALUES.len() - 1);
    }
}

pub fn move_to(coord: &Coord) {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(coord.x as i32, coord.y as i32);
    enigo.mouse_click(MouseButton::Left);
    sleep(Duration::from_millis(200));
    enigo.mouse_move_to(1748, 150);
}

pub fn right_click(coord: &Coord) {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(coord.x as i32, coord.y as i32);
    enigo.mouse_click(MouseButton::Right);
    sleep(Duration::from_millis(400));
    enigo.mouse_move_to(1748, 150);
}
