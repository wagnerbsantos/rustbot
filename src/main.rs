use captrs::*;
use hotkey::{use_hotkeys, FOOD_TIMER};
use model::*;
use shuteye::sleep;
use std::process::Command;
use std::time::{Duration, Instant};

use crate::attack::use_attack;
use crate::movement::use_movement;
use crate::screen::*;
use enigo::*;

mod attack;
mod hotkey;
mod model;
mod movement;
mod screen;

fn should_continue(image: &Image) -> u8 {
    if !has_color_at_position(image, ICON_SELECTED, ICON_SELECTED_COLOR, false) {
        println!("Tibia not selected");
        return 1;
    } else if has_color_at_position(image, LOGIN_SCREEN, LOGIN_SCREEN_COLOR, false) {
        println!("At login screen");
        if has_color_at_position(image, WAITING_EMAIL, WAITING_EMAIL_COLOR, false) {
            println!("Wating email");
            return 3;
        }
        return 2;
    }
    0
}

fn get_life(image: &Image) -> u8 {
    if has_greater_color_at_position(image, HIGH_LIFE_POS, LIFE_COLOR) {
        3
    } else if has_greater_color_at_position(image, MID_LIFE_POS, LIFE_COLOR) {
        2
    } else if has_greater_color_at_position(image, LOW_LIFE_POS, LIFE_COLOR) {
        1
    } else {
        0
    }
}

fn get_mana(image: &Image) -> u8 {
    if has_color_at_position(image, HIGH_MANA_POS, MANA_COLOR, false) {
        3
    } else if has_color_at_position(image, MID_MANA_POS, MANA_COLOR, false) {
        2
    } else if has_color_at_position(image, LOW_MANA_POS, MANA_COLOR, false) {
        1
    } else {
        0
    }
}

fn use_image(image: &Image, mut status: Status) -> Status {
    status.food_timer = status.food_timer - 1;
    status.ladder_cooldown = status.ladder_cooldown - 1;
    if !status.is_attacking {
        status.move_timer = status.move_timer - 1;
    }
    status.life = get_life(image);
    status.mana = get_mana(image);
    status.has_cap = get_has_cap(image);

    status
}

fn get_has_cap(image: &Image) -> bool {
    let mut cap_pos_1 = Coord { x: 1847, y: 285 };
    let cap_color = Color {
        r: 50,
        b: 30,
        g: 30,
    };
    let mut result = false;
    for _ in 0..6 {
        cap_pos_1.x = cap_pos_1.x + 1;
        if has_greater_color_at_position(image, &cap_pos_1, &cap_color) {
            result = true;
        }
    }
    result
}

fn run_gameloop() {
    let mut capturer = Capturer::new(0).unwrap();
    let bounds: Coord = new_coord(&capturer.geometry());
    print!("{:?}", &bounds.y);
    // let size = w as u64 * h as u64;
    let mut status: Status = Status {
        life: 3,
        mana: 3,
        food_timer: 2,
        move_timer: 10,
        next_waypoint: 0,
        is_attacking: false,
        is_moving: false,
        has_cap: true,
        ladder_cooldown: 10,
    };
    loop {
        let now = Instant::now();

        let result = capturer.capture_store_frame();
        match result {
            Err(e) => {
                print!("{:?}", e);
                continue;
            }
            _ => {}
        }
        let image = capturer.get_stored_frame();
        match image {
            None => {}
            Some(im) => {
                let image = Image {
                    bounds: &bounds,
                    pixels: im,
                };
                get_color_at_position(&image, &Coord { x: 542, y: 40 }, true);
                match should_continue(&image) {
                    0 => {
                        status = use_image(&image, status);
                        use_hotkeys(&status);
                        if status.food_timer < 0 {
                            status.food_timer = FOOD_TIMER
                        }
                        let enemy_count = use_attack(&image, &mut status);
                        sleep(Duration::from_millis(200));
                        if enemy_count == 0 {
                            use_movement(&image, &mut status);
                        }
                    }
                    1 => {
                        //Goto Tibia
                        // if has_color_at_position(&image, WINDOW_EXIST, WINDOW_EXIST_COLOR, false) {
                        //     let mut enigo = Enigo::new();
                        //     enigo.mouse_move_to(ICON_SELECTED.x.try_into().unwrap(), ICON_SELECTED.y.try_into().unwrap());
                        //     enigo.mouse_click(MouseButton::Left);
                        // } else {
                        //     run_tibia();
                        // }
                    }
                    2.. => {
                        //Use Password
                        let mut enigo = Enigo::new();
                        enigo.key_sequence("76509823tyu");
                        sleep(Duration::from_millis(2000));

                        enigo.mouse_move_to(
                            LOGIN_BUTTON.x.try_into().unwrap(),
                            LOGIN_BUTTON.y.try_into().unwrap(),
                        );
                        enigo.mouse_click(MouseButton::Left);
                        enigo.mouse_move_to(
                            BRAVE_BUTTON.x.try_into().unwrap(),
                            BRAVE_BUTTON.y.try_into().unwrap(),
                        );
                        enigo.mouse_click(MouseButton::Left);
                        sleep(Duration::from_millis(5000));
                        enigo.mouse_move_to(
                            EMAIL_LINK.x.try_into().unwrap(),
                            EMAIL_LINK.y.try_into().unwrap(),
                        );
                        enigo.mouse_click(MouseButton::Left);

                        sleep(Duration::from_millis(6000));
                        enigo.mouse_move_to(
                            FIRST_EMAIL.x.try_into().unwrap(),
                            FIRST_EMAIL.y.try_into().unwrap(),
                        );
                        enigo.mouse_click(MouseButton::Left);

                        sleep(Duration::from_millis(3000));
                        enigo.mouse_move_to(
                            EMAIL_CODE.x.try_into().unwrap(),
                            EMAIL_CODE.y.try_into().unwrap(),
                        );
                        enigo.mouse_click(MouseButton::Left);
                        sleep(Duration::from_millis(100));
                        enigo.mouse_click(MouseButton::Left);

                        sleep(Duration::from_millis(1000));
                        enigo.key_down(Key::Control);
                        enigo.key_sequence("c");
                        enigo.key_up(Key::Control);

                        sleep(Duration::from_millis(2000));
                        enigo.mouse_move_to(
                            DELETE_EMAIL.x.try_into().unwrap(),
                            DELETE_EMAIL.y.try_into().unwrap(),
                        );
                        enigo.mouse_click(MouseButton::Left);

                        enigo.mouse_move_to(
                            ICON_SELECTED.x.try_into().unwrap(),
                            ICON_SELECTED.y.try_into().unwrap(),
                        );
                        enigo.mouse_click(MouseButton::Left);

                        sleep(Duration::from_millis(1000));
                        enigo.mouse_move_to(
                            CODE_BOX.x.try_into().unwrap(),
                            CODE_BOX.y.try_into().unwrap(),
                        );
                        enigo.mouse_click(MouseButton::Left);
                        enigo.key_down(Key::Control);
                        enigo.key_sequence("v");
                        enigo.key_up(Key::Control);

                        sleep(Duration::from_millis(2000));
                        enigo.mouse_move_to(
                            SEND_CODE.x.try_into().unwrap(),
                            SEND_CODE.y.try_into().unwrap(),
                        );
                        enigo.mouse_click(MouseButton::Left);

                        sleep(Duration::from_millis(2000));
                        enigo.mouse_move_to(
                            CHAR_SELECT.x.try_into().unwrap(),
                            CHAR_SELECT.y.try_into().unwrap(),
                        );
                        enigo.mouse_click(MouseButton::Left);

                        sleep(Duration::from_millis(4000));
                    }
                }

                println!("{:?}", &status);
            }
        }

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        if elapsed.as_millis() < 1000 {
            sleep(Duration::from_millis(1000 - elapsed.as_millis() as u64));
        } else {
            sleep(Duration::from_millis(2000));
        }
        //break;
    }
}

fn main() {
    run_gameloop();
    // run_albion()
}

fn run_albion() {
    let mut enigo = Enigo::new();
    loop {
        enigo.key_sequence("y");
        sleep(Duration::from_millis(1000));
    }
}

fn run_tibia() {
    print!(
        "{:?}",
        Command::new("sh")
            .arg("-c")
            .arg("exec /home/kazye/Desktop/Tibia/Tibia")
            .spawn()
    );
}
