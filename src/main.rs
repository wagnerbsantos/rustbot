use captrs::*;
use hotkey::{use_hotkeys, FOOD_TIMER};
use model::*;
use shuteye::sleep;
use std::fs::File;
use std::io::BufReader;
use std::process::Command;
use std::time::{Duration, Instant};

use crate::attack::use_attack;
use crate::movement::use_movement;
use crate::screen::*;
use enigo::*;
use rodio::Source;

mod attack;
mod hotkey;
mod model;
mod movement;
mod screen;
mod waypoints;

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
    let mut life_pos = Coord {
        x: LIFE_BAR_START.x,
        y: LIFE_BAR_START.y,
    };
    for i in 0..9 {
        if !has_color_at_position(image, &life_pos, LIFE_BAR_COLOR, false) {
            return i as u8;
        }
        life_pos.x = life_pos.x + 10;
    }
    return 10;
}

fn get_mana(image: &Image) -> u8 {
    let mut mana_pos = Coord {
        x: MANA_BAR_START.x,
        y: MANA_BAR_START.y,
    };
    for i in 0..9 {
        if !has_color_at_position(image, &mana_pos, MANA_BAR_COLOR, false) {
            return i as u8;
        }
        mana_pos.x = mana_pos.x + 9;
    }
    return 10;
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
    status.has_full_mantra = get_has_full_mantra(image);
    status.healing_cooldown =
        has_color_at_position(image, HEALING_COOLDOWN_POS, HEALING_COOLDOWN_COLOR, false);
    status.mana_pot_cooldown =
        has_color_at_position(image, MANA_POT_COOLDOWN_POS, MANA_POT_COOLDOWN_COLOR, false);

    status
}

fn get_has_full_mantra(image: &Image) -> bool {
    let mantra_pos = Coord { x: 1029, y: 77 };
    let mantra_color = Color {
        r: 219,
        g: 154,
        b: 70,
    };
    return has_color_at_position(image, &mantra_pos, &mantra_color, false);
}

fn get_has_cap(image: &Image) -> bool {
    let mut cap_pos_1 = Coord { x: 1841, y: 287 };
    let cap_color = Color {
        r: 20,
        b: 30,
        g: 51,
    };
    let mut result = false;
    for _ in 0..10 {
        cap_pos_1.x = cap_pos_1.x - 1;
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
        has_full_mantra: false,
        is_monk: true,
        healing_cooldown: false,
        mana_pot_cooldown: false,
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
                //get_color_at_position(&image, ICON_SELECTED, true);
                match should_continue(&image) {
                    0 => {
                        status = use_image(&image, status);
                        use_hotkeys(&status);
                        if status.life == 1 {
                            playAudio();
                        }
                        if status.food_timer < 0 {
                            status.food_timer = FOOD_TIMER
                        }
                        // let enemy_count = use_attack(&image, &mut status);
                        // if enemy_count == 0 {
                        //     use_movement(&image, &mut status);
                        // }
                    }
                    1 => {
                        if has_color_at_position(&image, WINDOW_EXIST, WINDOW_EXIST_COLOR, false) {
                            // let mut enigo = Enigo::new();
                            // enigo.mouse_move_to(
                            //     ICON_SELECTED.x.try_into().unwrap(),
                            //     ICON_SELECTED.y.try_into().unwrap(),
                            // );
                            // enigo.mouse_click(MouseButton::Left);
                        } else {
                            //run_tibia();
                        }
                    }
                    2.. => {
                        break;
                        // 	//Use Password
                        // 	let mut enigo = Enigo::new();
                        // 	enigo.key_sequence("76509823tyu");
                        // 	sleep(Duration::from_millis(2000));

                        // 	enigo.mouse_move_to(
                        // 		LOGIN_BUTTON.x.try_into().unwrap(),
                        // 		LOGIN_BUTTON.y.try_into().unwrap(),
                        // 	);
                        // 	enigo.mouse_click(MouseButton::Left);
                        // 	enigo.mouse_move_to(
                        // 		BRAVE_BUTTON.x.try_into().unwrap(),
                        // 		BRAVE_BUTTON.y.try_into().unwrap(),
                        // 	);
                        // 	enigo.mouse_click(MouseButton::Left);
                        // 	sleep(Duration::from_millis(5000));
                        // 	enigo.mouse_move_to(
                        // 		EMAIL_LINK.x.try_into().unwrap(),
                        // 		EMAIL_LINK.y.try_into().unwrap(),
                        // 	);
                        // 	enigo.mouse_click(MouseButton::Left);

                        // 	sleep(Duration::from_millis(6000));
                        // 	enigo.mouse_move_to(
                        // 		FIRST_EMAIL.x.try_into().unwrap(),
                        // 		FIRST_EMAIL.y.try_into().unwrap(),
                        // 	);
                        // 	enigo.mouse_click(MouseButton::Left);

                        // 	sleep(Duration::from_millis(3000));
                        // 	enigo.mouse_move_to(
                        // 		EMAIL_CODE.x.try_into().unwrap(),
                        // 		EMAIL_CODE.y.try_into().unwrap(),
                        // 	);
                        // 	enigo.mouse_click(MouseButton::Left);
                        // 	sleep(Duration::from_millis(100));
                        // 	enigo.mouse_click(MouseButton::Left);

                        // 	sleep(Duration::from_millis(1000));
                        // 	enigo.key_down(Key::Control);
                        // 	enigo.key_sequence("c");
                        // 	enigo.key_up(Key::Control);

                        // 	sleep(Duration::from_millis(2000));
                        // 	enigo.mouse_move_to(
                        // 		DELETE_EMAIL.x.try_into().unwrap(),
                        // 		DELETE_EMAIL.y.try_into().unwrap(),
                        // 	);
                        // 	enigo.mouse_click(MouseButton::Left);

                        // 	enigo.mouse_move_to(
                        // 		ICON_SELECTED.x.try_into().unwrap(),
                        // 		ICON_SELECTED.y.try_into().unwrap(),
                        // 	);
                        // 	enigo.mouse_click(MouseButton::Left);

                        // 	sleep(Duration::from_millis(1000));
                        // 	enigo.mouse_move_to(
                        // 		CODE_BOX.x.try_into().unwrap(),
                        // 		CODE_BOX.y.try_into().unwrap(),
                        // 	);
                        // 	enigo.mouse_click(MouseButton::Left);
                        // 	enigo.key_down(Key::Control);
                        // 	enigo.key_sequence("v");
                        // 	enigo.key_up(Key::Control);

                        // 	sleep(Duration::from_millis(2000));
                        // 	enigo.mouse_move_to(
                        // 		SEND_CODE.x.try_into().unwrap(),
                        // 		SEND_CODE.y.try_into().unwrap(),
                        // 	);
                        // 	enigo.mouse_click(MouseButton::Left);

                        // 	sleep(Duration::from_millis(2000));
                        // 	enigo.mouse_move_to(
                        // 		CHAR_SELECT.x.try_into().unwrap(),
                        // 		CHAR_SELECT.y.try_into().unwrap(),
                        // 	);
                        // 	enigo.mouse_click(MouseButton::Left);

                        // 	sleep(Duration::from_millis(4000));
                    }
                }
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                println!("{:?}", &status);
            }
        }

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        if elapsed.as_millis() < 201 {
            sleep(Duration::from_millis(201 - elapsed.as_millis() as u64));
        } else {
            sleep(Duration::from_millis(1000));
        }
        //break;
    }
}

fn playAudio() {
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let welcome = File::open("src/resources/Tristam.ogg").unwrap();
    let source = rodio::Decoder::new(BufReader::new(welcome)).unwrap();
    stream_handle
        .play_raw(source.convert_samples())
        .expect("TODO: panic message");
    sleep(Duration::from_millis(1000));
}

fn main() {
    run_gameloop();
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
