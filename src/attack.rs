use crate::hotkey::click;
use crate::model::{Color, Coord, Image, Status};
use crate::screen::has_color_at_position;
use enigo::{Key};

pub const ENEMY_1_POS: Coord = Coord { x: 157, y: 62 };
pub const ENEMY_1_ATTACK: Coord = Coord { x: 4, y: 49 };
pub const ENEMY_OFFSET: u32 = 22;
pub const ENEMY_BAR_COLOR: Color = Color { r: 0, g: 0, b: 0 };
pub const ENEMY_ATTACK_COLOR: Color = Color { r: 255, g: 0, b: 0 };
pub const FOLLOW_BUTTON: Coord = Coord { x: 1902, y: 182 };
// pub const EMPTY_WEAPON_POSITION: Coord = Coord { x: 1760, y: 227 };
// pub const NO_WEAPON_2_POSITION: Coord = Coord { x: 1535, y: 286 };
// pub const WEAPON_2_EQUIPED_POSITION: Coord = Coord { x: 1534, y: 285 };
// pub const EMPTY_WEAPON_COLOR: Color = Color {
//     r: 118,
//     g: 120,
//     b: 123,
// };

pub const FOLLOW_BUTTON_COLOR_DISABLED: Color = Color {
    r: 201,
    g: 201,
    b: 201,
};

pub const LAST_ENEMY_HOTKEY: Key = Key::Layout('p');
pub const NEXT_ENEMY_HOTKEY: Key = Key::Layout('e');
// pub const EQUIP_SWORD_HOTKEY: Key = Key::Layout('1');
// pub const EQUIP_SPEAR_HOTKEY: Key = Key::Layout('2');

pub fn use_attack(image: &Image, status: &mut Status) -> i32 {
    // check_weapon(image);
    let enemy_count = count_enemies(image);
    if enemy_count > 0 {
        let target = get_attacker(image, enemy_count);
        if status.is_attacking && target.is_none() {
            loot();
            status.is_attacking = false;
        }
        use_target(target, status.no_dps, status.ladder_cooldown);
        set_follow(image);
        status.is_attacking = true;
        loot();
    } else {
        if status.is_attacking == true {
            status.is_attacking = false;
            status.is_moving = false;
            loot();
        }
    }
    enemy_count
}

// fn check_weapon(image: &Image) {
//     let has_no_weapon =
//         has_color_at_position(image, &EMPTY_WEAPON_POSITION, &EMPTY_WEAPON_COLOR, false);
//     let has_no_ranged_weapon = 
//         has_color_at_position(image, &NO_WEAPON_2_POSITION, &Color {r: 68, g: 68, b: 69}, false);
//     let has_ranged_weapon_equiped = 
//         has_color_at_position(image, &WEAPON_2_EQUIPED_POSITION, &Color {r: 41, g: 41, b: 41}, false);
//     if has_no_weapon {
//             click(EQUIP_SWORD_HOTKEY);
//     } else {
//         if !has_ranged_weapon_equiped && !has_no_ranged_weapon {
//             click(EQUIP_SPEAR_HOTKEY)
//         }
//     }
// }

pub fn loot() {
    click(Key::F12)
}

pub fn set_follow(image: &Image) {
    if has_color_at_position(image, &FOLLOW_BUTTON, &FOLLOW_BUTTON_COLOR_DISABLED, false, false) { 
        click(Key::Layout('\\'));
    }
}

pub fn use_target(target: Option<i32>, no_dps: bool, cooldown: i64) {
    match target {
        Some(t) => {
            if t > 2 {
                click(LAST_ENEMY_HOTKEY);
            }if no_dps && cooldown % 5 == 0{
                click(LAST_ENEMY_HOTKEY);
            }
        }
        None => {
            click(NEXT_ENEMY_HOTKEY);
        }
    }
}

pub fn count_enemies(image: &Image) -> i32 {
    let mut count = 0;
    for i in 0..10 {
        if has_color_at_position(
            image,
            &Coord {
                x: ENEMY_1_POS.x,
                y: ENEMY_1_POS.y + i * ENEMY_OFFSET,
            },
            &ENEMY_BAR_COLOR,
            false,
            false,
        ) {
            count += 1;
        }
    }
    count
}

pub fn get_attacker(image: &Image, enemy_count: i32) -> Option<i32> {
    let mut found: i32 = -1;
    for i in 0..enemy_count as u32 {
        if (has_color_at_position(
            image,
            &Coord {
                x: ENEMY_1_ATTACK.x,
                y: ENEMY_1_ATTACK.y + i * ENEMY_OFFSET,
            },
            &ENEMY_ATTACK_COLOR,
            false,
            false,
        )) {
            found = i as i32;
        }
    }
    if found == -1 {
        return None;
    }
    Some(found)
}
