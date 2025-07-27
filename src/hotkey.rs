use std::time::Duration;

use crate::model::Status;
use enigo::*;
use shuteye::sleep;
// TIMER
pub const FOOD_TIMER: i64 = 300;

//HOTKEYS
pub const WASTE_MANA_HOTKEY: Key = Key::PageUp;
pub const AOE_ATTACK_HOTKEY: Key = Key::Home;
pub const MANTRA_SKILL_HOTKEY: Key = Key::PageDown;
pub const SMALL_HEAL_HOTKEY: Key = Key::F1;
pub const HEAL_HOTKEY: Key = Key::F2;
pub const BIG_HEAL_HOTKEY: Key = Key::F3;
pub const BIG_MANA_HOTKEY: Key = Key::F5;
pub const MEDIUM_MANA_HOTKEY: Key = Key::F6;
pub const SMALL_MANA_HOTKEY: Key = Key::F7;
pub const FOOD_HOTKEY: Key = Key::F8;

pub fn use_hotkeys(status: &Status) {
    use_general_hotkeys(status);
}

pub fn use_general_hotkeys(status: &Status) {
    let mut done = 0;
    // items
    if !status.item_cooldown && done == 0 {
        if status.mana <= 10 && done == 0 {
            if status.big_mana_available {
                click(BIG_MANA_HOTKEY);
                done = 1;
            } else if status.medium_mana_available {
                click(MEDIUM_MANA_HOTKEY);
                done = 1;
            } else if status.small_mana_available {
                click(SMALL_MANA_HOTKEY);
                done = 1;
            }
            sleep(Duration::from_millis(50));
        }
        if status.mana <= 12 && done == 0 {
            if status.medium_mana_available {
                click(MEDIUM_MANA_HOTKEY);
                done = 1;
            } else if status.small_mana_available {
                click(SMALL_MANA_HOTKEY);
                done = 1;
            }
            sleep(Duration::from_millis(50));
        }
        if status.mana <= 16 && done == 0 {
            if status.small_mana_available {
                click(SMALL_MANA_HOTKEY);
                done = 1;
            }
            sleep(Duration::from_millis(50));
        }
    }
    if !status.healing_cooldown && done == 0 {
        if status.life <= 13 && done == 0 {
            click(BIG_HEAL_HOTKEY);
            done = 1;
        }
        if status.life <= 15 && done == 0 {
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
    if status.life <= 17 && status.is_attacking && !status.general_attack_cooldown {
        if status.has_full_mantra {
            click(MANTRA_SKILL_HOTKEY);
        } else if status.number_enemies >= 4 && !status.aoe_cooldown {
            click(AOE_ATTACK_HOTKEY);
        } else if !status.attack_cooldown {
            click(WASTE_MANA_HOTKEY); // attack
        }
    }
}

pub fn click(hotkey: Key) {
    let mut enigo = Enigo::new();
    enigo.key_click(hotkey);
}
