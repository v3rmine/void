// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! impl_id {
    ( $name:ident, $doc:expr$(,)? ) => {
        #[derive(
            Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
        )]
        #[doc = $doc]
        pub struct $name(u64);

        impl $name {
            /// Create a new id.
            pub const fn new(id: u64) -> Self {
                $name(id)
            }

            /// The value of the id.
            pub const fn value(&self) -> u64 {
                self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

macro_rules! enum_serialize {
    ( $name:ident -> $desc:expr, $( $value:ident => $str:expr $( ; $opt:expr )*, )+ ) => {
        #[allow(deprecated)]
        impl $name {
            /// String representation of the variant.
            pub fn as_str(&self) -> &'static str {
                match *self {
                    $( $name::$value => $str, )*
                }
            }
        }

        #[allow(deprecated)]
        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where S: Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        #[allow(deprecated)]
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where D: Deserializer<'de>,
            {
                let val = String::deserialize(deserializer)?;

                match val.as_str() {
                    $( $str $( | $opt )* => Ok($name::$value), )*
                    v => {
                        error!(target: "gitlab", concat!("unknown ", $desc, " from gitlab: {}"), v);
                        Err(D::Error::unknown_variant(v, &[$( $str, $( $opt, )* )*]))
                    },
                }
            }
        }
    };
}
