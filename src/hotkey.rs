use std::time::Duration;

use crate::model::Status;
use enigo::*;
use shuteye::sleep;

// TIMER
pub const FOOD_TIMER: i64 = 60;

//HOTKEYS
pub const MANA_POT_HOTKEY: Key = Key::Layout('c');
pub const WASTE_MANA_HOTKEY: Key = Key::F1;
pub const HEAL_HOTKEY: Key = Key::F2;
// pub const HIGH_HEAL_HOTKEY: Key = Key::Layout('8');
pub const LIFE_POT_HOTKEY: Key = Key::Layout('x');
pub const FOOD_HOTKEY: Key = Key::Layout('b');
pub const FOOD_HOTKEY_2: Key = Key::Layout('m');

pub fn use_hotkeys(status: &Status) {
    use_items_hotkeys(status);
}

pub fn use_items_hotkeys(status: &Status) {
    // items
    if status.life == 0 {
        click(LIFE_POT_HOTKEY)
    } else if status.mana <= 1 {
        click(MANA_POT_HOTKEY)
    }
    if status.food_timer == 0 {
        click(FOOD_HOTKEY);
        sleep(Duration::from_millis(100));
        click(FOOD_HOTKEY_2)
    }

    // spells
    if status.life < 3 && status.life > 0 {
        click(HEAL_HOTKEY)
    } else if status.mana == 3 {
        click(WASTE_MANA_HOTKEY);
    }
}

pub fn click(hotkey: Key) {
    let mut enigo = Enigo::new();
    enigo.key_click(hotkey);
}
