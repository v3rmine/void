#![allow(unused_imports)]

macro_rules! pub_mod_use {
    ($($name:ident),+) => {
        $(
        mod $name;
        pub use $name::*;
        )+
    };
}

pub_mod_use! {
    token,
    response,
    status,
    announcement,
    ge_item,
    event,
    resource,
    skill,
    drop_rate,
    monster,
    item,
    single_item,
    item_effect,
    craft,
    craft_skill,
    simple_item,
    item_type,
    map,
    map_content,
    map_content_type,
    character,
    inventory_slot,
    gold,
    message,
    character_movement_data,
    cooldown,
    destination_response,
    reason,
    slot_type,
    equip_request,
    fight,
    character_fight_data,
    result,
    drop,
    blocked_hits
}
