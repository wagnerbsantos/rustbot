use std::time::Duration;

use crate::model::Status;
use enigo::*;
use shuteye::sleep;
// TIMER
pub const FOOD_TIMER: i64 = 60;

//HOTKEYS
pub const WASTE_MANA_HOTKEY: Key = Key::F1;
pub const HEAL_HOTKEY: Key = Key::F2;
pub const BIG_HEAL_HOTKEY: Key = Key::F3;
pub const MANTRA_SKILL_HOTKEY: Key = Key::F4;
pub const MANA_POT_HOTKEY: Key = Key::F5;
pub const LIFE_POT_HOTKEY: Key = Key::F6;
pub const FOOD_HOTKEY: Key = Key::F7;

// pub const EQUIP_RANGED_HOTKEY: Key = Key::Layout('2');

pub fn use_hotkeys(status: &Status) {
    use_general_hotkeys(status);
    use_monk_hotkeys(status);
}

pub fn use_general_hotkeys(status: &Status) {
    // items
    if !status.mana_pot_cooldown {
        if status.life < 3 && status.life != 0 {
            click(LIFE_POT_HOTKEY);
            sleep(Duration::from_millis(50));
        } else if status.mana <= 5 && status.mana != 0 {
            click(MANA_POT_HOTKEY);
            sleep(Duration::from_millis(50));
        }
    }

    if status.food_timer == 0 {
        click(FOOD_HOTKEY);
    }
}

pub fn use_monk_hotkeys(status: &Status) {
    if !status.healing_cooldown && status.life != 0 {
        if status.life < 7 {
            click(BIG_HEAL_HOTKEY)
        } else if status.life < 10 {
            click(HEAL_HOTKEY)
        }
    }
}

pub fn click(hotkey: Key) {
    let mut enigo = Enigo::new();
    enigo.key_click(hotkey);
}
