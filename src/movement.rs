use crate::model::{Color, Coord, Image, Status};
use crate::screen::{get_color_positions_in_area, has_color_at_position, MAP_AREA};
use crate::waypoints::*;
use enigo::{Enigo, MouseButton, MouseControllable};
use shuteye::sleep;
use std::time::Duration;

pub const WAYPOINTS: [Color; 7] = [
    WAYPOINT_STAR,
    WAYPOINT_CHECK_MARK,
    WAYPOINT_BANNER,
    WAYPOINT_LOCK,
    WAYPOINT_QUESTIONMARK,
    WAYPOINT_MASK,
    WAYPOINT_MONEY,
];

pub const MAP_CENTER: Coord = Coord { x: 1806, y: 90 };
pub const WAYPOINT_OFFSET: u32 = 5;
pub const WAYPOINT_BORDER_COLOR: Color = Color {
    r: 70,
    g: 58,
    b: 50,
};

pub fn use_movement(image: &Image, status: &mut Status) {
    let coords = get_color_positions_in_area(
        image,
        MAP_AREA,
        WAYPOINTS.get(status.next_waypoint).unwrap(),
        false,
    );
    let mut sanitized_coords: Vec<Coord> = coords
        .iter()
        .filter_map(|coord| {
            if has_color_at_position(
                image,
                &Coord {
                    x: coord.x - WAYPOINT_OFFSET,
                    y: coord.y,
                },
                &WAYPOINT_BORDER_COLOR,
                true,
                false,
            ) && has_color_at_position(
                image,
                &Coord {
                    x: coord.x + WAYPOINT_OFFSET,
                    y: coord.y,
                },
                &WAYPOINT_BORDER_COLOR,
                true,
                false,
            ) && has_color_at_position(
                image,
                &Coord {
                    x: coord.x,
                    y: coord.y + WAYPOINT_OFFSET,
                },
                &WAYPOINT_BORDER_COLOR,
                true,
                false,
            ) && has_color_at_position(
                image,
                &Coord {
                    x: coord.x,
                    y: coord.y - WAYPOINT_OFFSET,
                },
                &WAYPOINT_BORDER_COLOR,
                true,
                false,
            ) {
                Some(Coord {
                    x: coord.x,
                    y: coord.y,
                })
            } else {
                None
            }
        })
        .collect();
    println!("{}", coords.len());
    println!("{}", sanitized_coords.len());
    if sanitized_coords.len() > 0 {
        let next = sanitized_coords.get(0).unwrap();
        println!("{:?}", next);
        let under_checkpoint =
            next.x == MAP_CENTER.x && ((next.y as i32 - MAP_CENTER.y as i32).abs() <= 1);

        if under_checkpoint && !status.is_attacking {
            status.next_waypoint = (status.next_waypoint + 1) % (WAYPOINTS.len() - 1);
            status.move_timer = 2;
            if status.ladder_cooldown <= 0 && under_checkpoint {
                right_click(&Coord { x: 950, y: 480 });
                status.ladder_cooldown = 3;
                sleep(Duration::from_millis(150));
            }
            println!("under checkpoint");
        }
        if status.move_timer <= 0 || !status.is_moving {
            move_to(next);
            status.move_timer = 7;
            status.is_moving = true;
        }
    } else {
        status.next_waypoint = (status.next_waypoint + 1) % (WAYPOINTS.len() - 1);
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

pub fn left_click() {
    let mut enigo = Enigo::new();
    enigo.mouse_click(MouseButton::Left);
}
