extern crate malachite_gmp;
extern crate malachite_native;
extern crate num;
extern crate rugint;
extern crate rust_wheels;

pub mod common;
pub mod integer {
    pub mod arithmetic {
        pub mod abs;
        pub mod add_i32;
        pub mod add_u32;
        pub mod neg;
        pub mod sub_i32;
        pub mod sub_u32;
    }
    pub mod comparison {
        pub mod eq;
        pub mod hash;
        pub mod ord;
        pub mod ord_abs;
        pub mod partial_ord_abs_i32;
        pub mod partial_ord_abs_natural;
        pub mod partial_ord_abs_u32;
        pub mod partial_ord_i32;
        pub mod partial_ord_natural;
        pub mod partial_ord_u32;
        pub mod partial_eq_i32;
        pub mod partial_eq_natural;
        pub mod partial_eq_u32;
        pub mod sign;
    }
    pub mod conversion {
        pub mod assign_i32;
        pub mod assign_i64;
        pub mod assign_natural;
        pub mod assign_u32;
        pub mod assign_u64;
        pub mod clone_and_assign;
        pub mod from_i32;
        pub mod from_i64;
        pub mod from_u32;
        pub mod from_u64;
        pub mod natural_assign_integer;
        pub mod to_i32;
        pub mod to_i64;
        pub mod to_natural;
        pub mod to_u32;
        pub mod to_u64;
    }
    pub mod logic {
        pub mod from_sign_and_limbs;
        pub mod get_bit;
        pub mod not;
        pub mod sign_and_limbs;
        pub mod significant_bits;
    }
}
pub mod natural {
    pub mod arithmetic {
        pub mod add;
        pub mod add_u32;
        pub mod even_odd;
        pub mod is_power_of_two;
        pub mod neg;
        pub mod shl_u32;
        pub mod sub;
        pub mod sub_u32;
    }
    pub mod comparison {
        pub mod eq;
        pub mod hash;
        pub mod ord;
        pub mod partial_ord_u32;
        pub mod partial_eq_u32;
    }
    pub mod conversion {
        pub mod assign_u32;
        pub mod assign_u64;
        pub mod clone_and_assign;
        pub mod from_u32;
        pub mod from_u64;
        pub mod to_integer;
        pub mod to_u32;
        pub mod to_u64;
    }
    pub mod logic {
        pub mod assign_bit;
        pub mod clear_bit;
        pub mod flip_bit;
        pub mod from_limbs;
        pub mod get_bit;
        pub mod limb_count;
        pub mod limbs;
        pub mod not;
        pub mod set_bit;
        pub mod significant_bits;
        pub mod trailing_zeros;
    }
}
