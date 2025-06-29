use std::time::Duration;

use crate::model::Status;
use enigo::*;
use shuteye::sleep;
// TIMER
pub const FOOD_TIMER: i64 = 300;

//HOTKEYS
pub const SMALL_HEAL_HOTKEY: Key = Key::F1;
pub const HEAL_HOTKEY: Key = Key::F2;
pub const BIG_HEAL_HOTKEY: Key = Key::F3;
pub const MANTRA_SKILL_HOTKEY: Key = Key::F4;
pub const BIG_MANA_HOTKEY: Key = Key::F5;
pub const MEDIUM_MANA_HOTKEY: Key = Key::F6;
pub const SMALL_MANA_HOTKEY: Key = Key::F7;
pub const FOOD_HOTKEY: Key = Key::F8;

// pub const EQUIP_RANGED_HOTKEY: Key = Key::Layout('2');

pub fn use_hotkeys(status: &Status) {
    if status.life == 0 && status.mana == 0 {
        return;
    }
    use_general_hotkeys(status);
    //use_monk_hotkeys(status);
}

pub fn use_general_hotkeys(status: &Status) {
    let mut done = 0;
    // items
    if !status.item_cooldown && done == 0 {
        if status.mana <= 12 && done == 0 {
            if status.big_mana_available {
                click(BIG_MANA_HOTKEY);
            } else if status.medium_mana_available {
                click(MEDIUM_MANA_HOTKEY);
            } else if status.small_mana_available {
                click(SMALL_MANA_HOTKEY);
            }
            sleep(Duration::from_millis(50));
            done = 1;
        }
        if status.mana <= 14 && done == 0 {
            if status.medium_mana_available {
                click(MEDIUM_MANA_HOTKEY);
            } else if status.small_mana_available {
                click(SMALL_MANA_HOTKEY);
            }
            sleep(Duration::from_millis(50));
            done = 1;
        }
        if status.mana <= 16 && done == 0 {
            if status.small_mana_available {
                click(SMALL_MANA_HOTKEY);
            }
            sleep(Duration::from_millis(50));
            done = 1;
        }
    }
    if !status.healing_cooldown && done == 0 {
        // monk
        if status.life <= 15 && done == 0 {
            click(BIG_HEAL_HOTKEY);
            done = 1;
        }
        if status.life <= 16 && done == 0 {
            click(HEAL_HOTKEY);
            done = 1;
        }
        if status.life <= 17 && done == 0 {
            click(SMALL_HEAL_HOTKEY);
            done = 1;
        }
    }
    if status.food_timer == 0 && done == 0 {
        click(FOOD_HOTKEY);
        done = 1;
    }
}

pub fn use_monk_hotkeys(status: &Status) {
    if !status.healing_cooldown && status.life != 0 {
        if status.life < 8 {
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
