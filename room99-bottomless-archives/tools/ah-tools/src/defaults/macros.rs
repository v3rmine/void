#[macro_export]
macro_rules! test_params {
    ($param:expr, $closure:expr, $($i:ident),*) => {
        move || -> Result<String, LocalErr> {
            let params: std::collections::HashMap<&str, &str> = $param;
            $(let $i = params.get(stringify!($i)).ok_or(format!("{} not found!", stringify!($i)).as_str())?.clone();)*
            $closure($($i),*)
        };
    };
    ($param:expr, $closure:expr, [$($i:ident),*], [$($o:ident),*]) => {
        move || -> Result<String, LocalErr> {
            let params: std::collections::HashMap<&str, &str> = $param;
            $(let $i = params.get(stringify!($i)).ok_or(format!("{} not found!", stringify!($i)).as_str())?.clone();)*
            $(let $o = match params.get(stringify!($o)) { Some(x) => Some(x.clone()), None => None };)*
            $closure(($($i),*), ($($o),*))
        };
    };
}
