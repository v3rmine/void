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
    action_move,
    action_equip_item,
    action_unequip_item,
    action_fight,
    action_gathering,
    action_crafting,
    action_deposit_bank,
    action_deposit_bank_gold,
    action_recycling,
    action_withdraw_bank,
    action_withdraw_bank_gold,
    action_ge_buy_item,
    action_ge_sell_item,
    action_accept_new_task,
    action_complete_task,
    action_task_exchange,
    action_delete_item,
    get_all_characters_logs,
    get_my_characters
}
