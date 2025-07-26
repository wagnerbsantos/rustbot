use crate::{model::*, screen::*};

pub fn use_image(image: &Image, mut status: Status) -> Status {
    status.food_timer = status.food_timer - 1;
    status.ladder_cooldown = status.ladder_cooldown - 1;
    if !status.is_attacking {
        status.move_timer = status.move_timer - 1;
    }
    status.life = get_life(image);
    status.mana = get_mana(image);
    status.has_cap = get_has_cap(image);
    status.has_full_mantra = get_has_full_mantra(image);
    status.healing_cooldown = has_color_at_position(
        image,
        HEALING_COOLDOWN_POS,
        HEALING_COOLDOWN_COLOR,
        false,
        false,
    );
    status.heal_other_cooldown = get_item_on_cooldown_by_slot(image, 2);
    status.item_cooldown = get_item_on_cooldown_by_slot(image, 16);
    status.big_mana_available = get_item_available_by_slot(image, 15);
    status.medium_mana_available = get_item_available_by_slot(image, 16);
    status.small_mana_available = get_item_available_by_slot(image, 17);
    status.knight_lowlife = get_ally_lowlife(image);

    status
}

pub fn should_continue(image: &Image) -> u8 {
    if !has_color_at_position(image, ICON_SELECTED, ICON_SELECTED_COLOR, false, false)
        || !has_color_at_position(image, SCREEN_SELECTED, SCREEN_SELECTED_COLOR, false, false)
    {
        println!("Tibia not selected");
        return 1;
    }
    0
}

fn get_life(image: &Image) -> u8 {
    let mut life_pos = Coord {
        x: LIFE_BAR_START.x,
        y: LIFE_BAR_START.y,
    };
    let mut life = 0;
    for _ in 0..22 {
        if has_color_at_position(image, &life_pos, LIFE_BAR_COLOR, false, true) {
            life = life + 1;
        }
        life_pos.x = life_pos.x + 4;
    }
    return life;
}

fn get_mana(image: &Image) -> u8 {
    let mut mana_pos = Coord {
        x: MANA_BAR_START.x,
        y: MANA_BAR_START.y,
    };
    let mut mana = 0;
    for _ in 0..22 {
        if has_color_at_position(image, &mana_pos, MANA_BAR_COLOR, false, true) {
            mana = mana + 1;
        }
        mana_pos.x = mana_pos.x + 4;
    }
    return mana;
}

fn get_has_full_mantra(image: &Image) -> bool {
    let mantra_pos = Coord { x: 1029, y: 77 };
    let mantra_color = Color {
        r: 219,
        g: 154,
        b: 70,
    };
    return has_color_at_position(image, &mantra_pos, &mantra_color, false, false);
}

fn get_ally_lowlife(image: &Image) -> bool {
    let ally_pos = Coord { x: 203, y: 64 };
    let target_health_color = Color { r: 190, g: 0, b: 0 };
    return has_greater_color_at_position(image, &ally_pos, &target_health_color);
}

fn get_has_cap(image: &Image) -> bool {
    let mut cap_pos_1 = Coord { x: 1841, y: 287 };
    let cap_color = Color {
        r: 20,
        b: 30,
        g: 51,
    };
    let mut result = false;
    for _ in 0..10 {
        cap_pos_1.x = cap_pos_1.x - 1;
        if has_greater_color_at_position(image, &cap_pos_1, &cap_color) {
            result = true;
        }
    }
    result
}

fn get_item_available_by_slot(image: &Image, slot: u32) -> bool {
    let left_bar_slot_start = Coord { x: 353, y: 109 };
    let next_slot_offset = 36;
    let unavailable_color = Color {
        r: 54,
        g: 54,
        b: 54,
    };
    let target_coord = Coord {
        x: left_bar_slot_start.x,
        y: (left_bar_slot_start.y + next_slot_offset * slot),
    };
    return !has_color_at_position(image, &target_coord, &unavailable_color, false, false);
}

fn get_item_on_cooldown_by_slot(image: &Image, slot: u32) -> bool {
    let left_bar_slot_start = Coord { x: 367, y: 124 };
    let next_slot_offset = 36;
    let cooldown_color = MANA_POT_COOLDOWN_COLOR;
    let target_coord = Coord {
        x: left_bar_slot_start.x,
        y: (left_bar_slot_start.y + next_slot_offset * slot),
    };
    return has_color_at_position(image, &target_coord, &cooldown_color, false, false);
}
