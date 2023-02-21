use captrs::*;
use hotkey::{use_hotkeys, FOOD_TIMER};
use model::*;
use shuteye::sleep;
use std::time::{Duration, Instant};

use crate::{
    hotkey::{click, WASTE_MANA_HOTKEY},
    screen::*,
};
mod hotkey;
mod model;
mod screen;

fn should_continue(image: &Image) -> bool {
    if !has_color_at_position(image, ICON_SELECTED, ICON_SELECTED_COLOR) {
        println!("Tibia not selected");
        return false;
    } else if has_color_at_position(image, LOGIN_SCREEN, LOGIN_SCREEN_COLOR) {
        println!("At login screen");
        return false;
    }
    return true;
}

fn get_life(image: &Image) -> u8 {
    if has_greater_color_at_position(image, HIGH_LIFE_POS, LIFE_COLOR) {
        return 3;
    } else if has_greater_color_at_position(image, MID_LIFE_POS, LIFE_COLOR) {
        return 2;
    } else if has_greater_color_at_position(image, LOW_LIFE_POS, LIFE_COLOR) {
        return 1;
    } else {
        return 0;
    }
}

fn get_mana(image: &Image) -> u8 {
    if has_color_at_position(image, HIGH_MANA_POS, MANA_COLOR) {
        return 3;
    } else if has_color_at_position(image, MID_MANA_POS, MANA_COLOR) {
        return 2;
    } else if has_color_at_position(image, LOW_MANA_POS, MANA_COLOR) {
        return 1;
    } else {
        return 0;
    }
}

fn use_image(image: &Image, mut status: Status) -> Status {
    status.food_timer = status.food_timer - 1;
    status.life = get_life(image);
    status.mana = get_mana(image);

    return status;
}

fn main() {
    let mut capturer = Capturer::new(0).unwrap();
    let bounds: Coord = new_coord(&capturer.geometry());
    // let size = w as u64 * h as u64;
    let mut status: Status = Status {
        life: 3,
        mana: 3,
        food_timer: 2,
    };
    loop {
        click(WASTE_MANA_HOTKEY);
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
                if should_continue(&image) {
                    status = use_image(&image, status);
                    use_hotkeys(&status);
                    if status.food_timer < 0 {
                        status.food_timer = FOOD_TIMER
                    }
                } else {
                    println!("skip");
                }

                println!("{:?}", &status);
            }
        }

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        sleep(Duration::from_millis(1300 - elapsed.as_millis() as u64));
    }
}
