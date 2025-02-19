use crate::hotkey::click;
use crate::model::{Color, Coord, Image, Status};
use crate::movement::move_to;
use crate::screen::has_color_at_position;
use enigo::{Enigo, Key, KeyboardControllable, MouseButton, MouseControllable};
use shuteye::sleep;
use std::time::Duration;

pub const ENEMY_1_POS: Coord = Coord { x: 157, y: 62 };
pub const ENEMY_1_ATTACK: Coord = Coord { x: 4, y: 49 };
pub const ENEMY_OFFSET: u32 = 22;
pub const ENEMY_BAR_COLOR: Color = Color { r: 0, g: 0, b: 0 };
pub const ENEMY_ATTACK_COLOR: Color = Color { r: 255, g: 0, b: 0 };
pub const FOLLOW_BUTTON: Coord = Coord { x: 1902, y: 182 };
pub const EMPTY_WEAPON_POSITION: Coord = Coord { x: 1760, y: 227 };
pub const NO_WEAPON_2_POSITION: Coord = Coord { x: 1535, y: 286 };
pub const WEAPON_2_EQUIPED_POSITION: Coord = Coord { x: 1534, y: 285 };
pub const EMPTY_WEAPON_COLOR: Color = Color {
    r: 118,
    g: 120,
    b: 123,
};

pub const FOLLOW_BUTTON_COLOR_DISABLED: Color = Color {
    r: 201,
    g: 201,
    b: 201,
};

pub const LAST_ENEMY_HOTKEY: Key = Key::Layout('p');
pub const NEXT_ENEMY_HOTKEY: Key = Key::Layout('e');
pub const EQUIP_SWORD_HOTKEY: Key = Key::Layout('1');
pub const EQUIP_SPEAR_HOTKEY: Key = Key::Layout('2');

struct LootCoords;
impl LootCoords {
    pub const LOOT_NW: Coord = Coord { x: 890, y: 400 };
    pub const LOOT_N: Coord = Coord { x: 950, y: 400 };
    pub const LOOT_NE: Coord = Coord { x: 1025, y: 400 };
    pub const LOOT_W: Coord = Coord { x: 890, y: 480 };
    pub const LOOT_E: Coord = Coord { x: 1025, y: 480 };
    pub const LOOT_SW: Coord = Coord { x: 890, y: 550 };
    pub const LOOT_S: Coord = Coord { x: 950, y: 550 };
    pub const LOOT_SE: Coord = Coord { x: 1025, y: 550 };
}
impl LootCoords {
    const VALUES: [Coord; 8] = [
        Self::LOOT_NW,
        Self::LOOT_N,
        Self::LOOT_NE,
        Self::LOOT_E,
        Self::LOOT_W,
        Self::LOOT_SW,
        Self::LOOT_S,
        Self::LOOT_SE,
    ];
}
pub fn use_attack(image: &Image, status: &mut Status) -> i32 {
    check_weapon(image);
    let enemy_count = count_enemies(image);
    if enemy_count > 0 {
        let target = get_attacker(image, enemy_count);
        if status.is_attacking && target.is_none() {
            loot();
            status.is_attacking = false;
        }
        use_target(target);
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

fn check_weapon(image: &Image) {
    let has_no_weapon =
        has_color_at_position(image, &EMPTY_WEAPON_POSITION, &EMPTY_WEAPON_COLOR, false);
    let has_no_ranged_weapon = 
        has_color_at_position(image, &NO_WEAPON_2_POSITION, &Color {r: 68, g: 68, b: 69}, false);
    let has_ranged_weapon_equiped = 
        has_color_at_position(image, &WEAPON_2_EQUIPED_POSITION, &Color {r: 41, g: 41, b: 41}, false);
    if has_no_weapon {
            click(EQUIP_SWORD_HOTKEY);
    } else {
        if !has_ranged_weapon_equiped && !has_no_ranged_weapon {
            click(EQUIP_SPEAR_HOTKEY)
        }
    }
}

pub fn loot() {
    click(Key::F12)
    // let mut enigo = Enigo::new();
    // enigo.key_down(Key::Shift);
    // for loot in LootCoords::VALUES.iter() {
    //     quick_loot(loot, &mut enigo);
    //     sleep(Duration::from_millis(100));
    // }
    // enigo.key_up(Key::Shift);
    // sleep(Duration::from_millis(200));
    // enigo.mouse_move_to(1748, 150);
}

pub fn quick_loot(coord: &Coord, enigo: &mut Enigo) {
    enigo.mouse_move_to(coord.x as i32, coord.y as i32);
    enigo.mouse_click(MouseButton::Right);
}

pub fn set_follow(image: &Image) {
    if has_color_at_position(image, &FOLLOW_BUTTON, &FOLLOW_BUTTON_COLOR_DISABLED, false) {
        move_to(&FOLLOW_BUTTON);
    }
}

pub fn use_target(target: Option<i32>) {
    match target {
        Some(t) => {
            if t > 2 {
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
        )) {
            found = i as i32;
        }
    }
    if found == -1 {
        return None;
    }
    Some(found)
}
