macro_rules! export_mod {
    ($($i:ident),+) => {
        $(pub mod $i;)+
    };
}

export_mod!(add, pop, push);
