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
pub const BIG_HEAL_HOTKEY: Key = Key::F3;
pub const MANTRA_SKILL_HOTKEY: Key = Key::F4;
// pub const HIGH_HEAL_HOTKEY: Key = Key::Layout('8');
pub const LIFE_POT_HOTKEY: Key = Key::Layout('x');
pub const FOOD_HOTKEY: Key = Key::Layout('b');
pub const FOOD_HOTKEY_2: Key = Key::Layout('m');
// pub const EQUIP_RANGED_HOTKEY: Key = Key::Layout('2');

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
    if status.life == 0 {
        click(LIFE_POT_HOTKEY)
    } else if status.mana <= 1 {
        click(MANA_POT_HOTKEY)
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
    if status.life == 2 {
        click(HEAL_HOTKEY)
    } else if status.life < 2 {
        click(BIG_HEAL_HOTKEY)
    }
    if status.mana >= 3 && status.ladder_cooldown % 3 == 0{
        if status.is_attacking {
            click(WASTE_MANA_HOTKEY);
        } else {
            click(Key::F11)
        }
    }
}

pub fn use_monk_hotkeys(status: &Status) {
    if status.life < 3 && status.is_attacking && status.ladder_cooldown % 3 == 0 {
        if status.has_full_mantra {
            click(MANTRA_SKILL_HOTKEY);
        } else {
            click(WASTE_MANA_HOTKEY); // attack
        }
    }

    // spells
    if status.life == 1 {
        click(HEAL_HOTKEY)
    } else if status.life < 1 {
        click(BIG_HEAL_HOTKEY)
    }
    if status.mana >= 3 && status.ladder_cooldown % 3 == 0{
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
