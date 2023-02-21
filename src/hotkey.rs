use crate::model::Status;
use enigo::*;

// TIMER
pub const FOOD_TIMER: i64 = 100;

//HOTKEYS
// pub const MANA_POT_HOTKEY: Key = Key::Layout('c');
pub const WASTE_MANA_HOTKEY: Key = Key::F1;
// pub const HEAL_HOTKEY: Key = Key::Layout('9');
// pub const HIGH_HEAL_HOTKEY: Key = Key::Layout('8');
// pub const LIFE_POT_HOTKEY: Key = Key::Layout('x');
// pub const FOOD_HOTKEY: Key = Key::Layout('b');

pub fn use_hotkeys(status: &Status) {
    use_items_hotkeys(status);
}

pub fn use_items_hotkeys(status: &Status) {
    if status.life == 0 {
        //enigo.key_click(LIFE_POT_HOTKEY)
    } else if status.mana <= 2 {
        //enigo = click(enigo, MANA_POT_HOTKEY)
    } else if status.food_timer == 0 {
        //enigo = click(enigo, FOOD_HOTKEY)
    } else if status.life == 2 {
        //enigo = click(enigo, HEAL_HOTKEY)
    } else if status.mana == 3 {
        click(WASTE_MANA_HOTKEY);
    }
}

pub fn click(hotkey: Key) {
    let mut enigo = Enigo::new();
    enigo.key_click(hotkey);
}
