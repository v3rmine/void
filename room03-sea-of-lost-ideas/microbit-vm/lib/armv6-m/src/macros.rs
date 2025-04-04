#[macro_export]
macro_rules! values_enum {
    ($name:ident, $repr:ty, {$($i:ident($d:ty) => $v:expr),*}) => {
        pub enum $name {
            $($i($d)),*
        }

        impl AsRef<$repr> for $name {
            fn as_ref(&self) -> &$repr {
                match self {
                    $($name::$i(_) => &$v),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! parse_bits {
    () => { unimplemented!() };
    (
        ($pat:expr, $patlen:expr),
        $blck:expr,
        $field:ident,
        $res:expr
    ) => {
        ::nom::combinator::map(
            ::nom::sequence::tuple((
                ::nom::bits::streaming::tag($pat, $patlen),
                ::nom::bits::streaming::take($blck)
            )),
            |(_, $field)| $res
        )
    };
    (
        ($pat:expr, $patlen:expr),
        ($($blck:expr),+),
        ($($fields:ident),+),
        $res:expr
    ) => {
        ::nom::combinator::map(
            ::nom::sequence::tuple((
                ::nom::bits::streaming::tag($pat, $patlen),
                ::nom::sequence::tuple(($(::nom::bits::streaming::take($blck)),+))
            )),
            |(_, ($($fields),+))| $res
        )
    };
}