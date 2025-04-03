macro_rules! export_crate_macro {
    ($name:ident $body:tt) => {
		macro_rules! $name $body
		pub(crate) use $name;
	};
}
pub(crate) use export_crate_macro;

export_crate_macro!(export_scoped_macro {
    ($name:ident $body:tt) => {
        macro_rules! $name $body
        pub(super) use $name;
    }
});

export_crate_macro! (pub_use_mod {
    ($module_name:ident) => {
        mod $module_name;
        pub use $module_name::*;
    };
});
