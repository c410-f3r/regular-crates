macro_rules! create_enum_with_list {
    (
        $err:expr;

        $(#[$mac:meta])*
        $v:vis enum $enum_ident:ident {
            $first_variant_ident:ident, $first_variant_str:literal;
            $($variant_ident:ident, $variant_str:literal;)*
        }
    ) => {
        $(#[$mac])*
        $v enum $enum_ident {
          $first_variant_ident,
          $($variant_ident,)*
        }

        impl $enum_ident {
            #[inline]
            pub(crate) const fn list() -> &'static str {
                concat!(
                    $first_variant_str,
                    $(", ", $variant_str,)*
                )
            }
        }

        impl core::str::FromStr for $enum_ident {
            type Err = crate::Error;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $first_variant_str => Self::$first_variant_ident,
                    $($variant_str => Self::$variant_ident,)*
                    _ => return Err($err),
                })
            }
        }
    }
}
