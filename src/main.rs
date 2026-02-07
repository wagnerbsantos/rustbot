use captrs::*;
use hotkey::{use_hotkeys, FOOD_TIMER};
use model::*;
use shuteye::sleep;
use std::fs::File;
use std::io::BufReader;
use std::time::{Duration, Instant};

use crate::attack::{loot, use_attack};
use crate::hotkey::click;
use crate::movement::use_movement;
use crate::{information::*, screen::*};
use enigo::*;
use rodio::Source;

mod attack;
mod hotkey;
mod information;
mod model;
mod movement;
mod screen;
mod waypoints;

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
        healing_cooldown: false,
        big_mana_available: false,
        medium_mana_available: false,
        item_cooldown: false,
        small_mana_available: false,
        attack_cooldown: false,
        general_attack_cooldown: false,
        aoe_cooldown: false,
        mantra_cooldown: false,
        no_dps: false,
        number_enemies: 0,
        knight_lowlife: false,
        heal_other_cooldown: false,
        auto_hunt: false,
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
                        if status.food_timer <= 0 {
                            click(Key::F8);
                            status.food_timer = FOOD_TIMER;
                        }
                        if status.ladder_cooldown % 8 == 0 && status.auto_hunt {
                            let enemy_count = use_attack(&image, &mut status);
                            if enemy_count == 0 {
                                loot();
                                sleep(Duration::from_millis(250 as u64));
                                use_movement(&image, &mut status);
                            }
                        }
                        println!("{:?}", status);
                    }
                    1 => {
                        if has_color_at_position(
                            &image,
                            WINDOW_EXIST,
                            WINDOW_EXIST_COLOR,
                            false,
                            false,
                        ) {
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
                    }
                }
            }
        }

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        if elapsed.as_millis() < 125 {
            sleep(Duration::from_millis(125 - elapsed.as_millis() as u64));
        }
        //break;
    }
}

pub fn play_audio() {
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
