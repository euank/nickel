//! Load the Nickel standard library in strings at compile-time.

use crate::term::make as mk_term;
use crate::term::RichTerm;

pub const BUILTIN: (&str, &str) = (
    "<stdlib/builtin.ncl>",
    include_str!("../stdlib/builtin.ncl"),
);
pub const CONTRACT: (&str, &str) = (
    "<stdlib/contract.ncl>",
    include_str!("../stdlib/contract.ncl"),
);
pub const ARRAY: (&str, &str) = ("<stdlib/array>", include_str!("../stdlib/array.ncl"));
pub const RECORD: (&str, &str) = ("<stdlib/record>", include_str!("../stdlib/record.ncl"));
pub const STRING: (&str, &str) = ("<stdlib/string>", include_str!("../stdlib/string.ncl"));
pub const NUM: (&str, &str) = ("<stdlib/num>", include_str!("../stdlib/num.ncl"));
pub const FUNCTION: (&str, &str) = ("<stdlib/function>", include_str!("../stdlib/function.ncl"));
pub const INTERNALS: (&str, &str) = (
    "<stdlib/internals>",
    include_str!("../stdlib/internals.ncl"),
);
pub const COMPAT: (&str, &str) = ("<stdlib/compat>", include_str!("../stdlib/compat.ncl"));

/// Return the list `(name, source_code)` of all the stdlib modules.
pub fn modules() -> Vec<(&'static str, &'static str)> {
    vec![
        BUILTIN, CONTRACT, ARRAY, RECORD, STRING, NUM, FUNCTION, INTERNALS, COMPAT,
    ]
}

macro_rules! generate_accessor {
    ($value:ident) => {
        pub fn $value() -> RichTerm {
            mk_term::var(format!("${}", stringify!($value)))
        }
    };
}

/// Accessors to the builtin contracts.
pub mod contract {
    use super::*;

    // `dyn` is a reserved keyword in rust
    pub fn dynamic() -> RichTerm {
        mk_term::var("$dyn")
    }

    generate_accessor!(num);
    generate_accessor!(bool);
    generate_accessor!(string);
    generate_accessor!(array);
    generate_accessor!(func);
    generate_accessor!(forall_var);
    generate_accessor!(fail);
    generate_accessor!(enums);
    generate_accessor!(enum_fail);
    generate_accessor!(record);
    generate_accessor!(dyn_record);
    generate_accessor!(record_extend);
    generate_accessor!(forall_tail);
    generate_accessor!(dyn_tail);
    generate_accessor!(empty_tail);
}

pub mod internals {
    use super::*;

    generate_accessor!(push_default);
    generate_accessor!(push_force);
}
