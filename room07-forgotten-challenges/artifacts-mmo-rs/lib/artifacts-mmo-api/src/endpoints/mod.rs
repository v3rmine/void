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
    get_status,
    my_characters,
    my_account,
    characters,
    maps,
    items,
    monsters,
    resources,
    events,
    grand_exchange,
    accounts,
    token
}
