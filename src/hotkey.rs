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
pub const FOOD_HOTKEY_2: Key = Key::F8;

pub fn use_hotkeys(status: &Status) {
    use_general_hotkeys(status);
    if status.is_monk {
        use_monk_hotkeys(status);
    } else {    
        use_items_hotkeys(status);
    }
}

pub fn use_general_hotkeys(status: &Status) {
    // items
    if !status.mana_pot_cooldown {
        if status.mana <= 4 && status.mana != 0 {
            click(MANA_POT_HOTKEY);
            sleep(Duration::from_millis(50));
        }
    }
    if status.food_timer == 0 {
        click(FOOD_HOTKEY);
        sleep(Duration::from_millis(100));
        click(FOOD_HOTKEY_2);
        // sleep(Duration::from_millis(100));
        // click(EQUIP_RANGED_HOTKEY);
    }
}

pub fn use_items_hotkeys(status: &Status) {
    if !status.healing_cooldown && status.life != 0 {
        if status.life < 7 {
            click(BIG_HEAL_HOTKEY)
        } else if status.life < 9 {
            click(HEAL_HOTKEY)
        }
    }
    if status.mana >= 8 && status.ladder_cooldown % 3 == 0{
        if status.is_attacking {
            click(WASTE_MANA_HOTKEY);
        } else {
            click(Key::F11)
        }
    }
}

pub fn use_monk_hotkeys(status: &Status) {
    if status.life < 9 && status.is_attacking && status.ladder_cooldown % 3 == 0 {
        if status.has_full_mantra {
            click(MANTRA_SKILL_HOTKEY);
        } else {
            click(WASTE_MANA_HOTKEY); // attack
        }
    }
    if status.mana > 0 {
    // spells
        if !status.healing_cooldown && status.life != 0 {
            if status.life < 6 {
                click(BIG_HEAL_HOTKEY)
            } else if status.life < 8 {
                click(HEAL_HOTKEY)
            }
        }
    }
    if status.mana >= 8 && status.ladder_cooldown % 3 == 0{
        if status.is_attacking {
            click(WASTE_MANA_HOTKEY);
        } else {
            click(Key::F11)
        }
    }
}

pub fn click(hotkey: Key) {
    let mut enigo = Enigo::new();
    enigo.key_click(hotkey);
}
