use crate::Float;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::bench::bucketers::{float_size, Bucketer};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use std::cmp::max;

pub fn pair_1_float_complexity_bucketer<T>(var_name: &str) -> Bucketer<(Float, T)> {
    Bucketer {
        bucketing_function: &|(x, _)| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn pair_2_float_complexity_bucketer<T>(var_name: &str) -> Bucketer<(T, Float)> {
    Bucketer {
        bucketing_function: &|(_, x)| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn float_complexity_bucketer(var_name: &str) -> Bucketer<Float> {
    Bucketer {
        bucketing_function: &|x| usize::exact_from(x.complexity()),
        bucketing_label: format!("{var_name}.complexity()"),
    }
}

pub fn pair_float_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, Float)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), y.complexity())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.complexity())"),
    }
}

pub fn pair_2_pair_float_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, Float))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| usize::exact_from(max(x.complexity(), y.complexity())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.complexity())"),
    }
}

pub fn pair_float_integer_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, Integer)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), y.significant_bits())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_float_natural_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, Natural)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), y.significant_bits())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_float_rational_max_complexity_bucketer<'a>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, Rational)> {
    Bucketer {
        bucketing_function: &|(x, y)| usize::exact_from(max(x.complexity(), y.significant_bits())),
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_float_integer_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, Integer))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_float_natural_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, Natural))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_float_rational_max_complexity_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, Rational))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_float_unsigned_max_complexity_bucketer<'a, T, U: PrimitiveUnsigned>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_float_signed_max_complexity_bucketer<'a, T, U: PrimitiveSigned>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.complexity(), y.significant_bits()))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_pair_float_primitive_float_max_complexity_bucketer<'a, T, U: PrimitiveFloat>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| {
            usize::exact_from(max(x.complexity(), u64::exact_from(float_size(*y))))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_float_primitive_float_max_complexity_bucketer<'a, T: PrimitiveFloat>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (Float, T)> {
    Bucketer {
        bucketing_function: &|(x, y)| {
            usize::exact_from(max(x.complexity(), u64::exact_from(float_size(*y))))
        },
        bucketing_label: format!("max({x_name}.complexity(), {y_name}.significant_bits())"),
    }
}

pub fn pair_2_max_triple_1_float_complexity_triple_2_bucketer<'a, T, U>(
    x_name: &'a str,
    p_name: &'a str,
) -> Bucketer<'a, (T, (Float, u64, U))> {
    Bucketer {
        bucketing_function: &|(_, (x, p, _))| usize::exact_from(max(x.complexity(), *p)),
        bucketing_label: format!("max({x_name}.complexity(), {p_name})"),
    }
}

pub fn pair_2_max_pair_1_complexity_pair_2_bucketer<'a, T>(
    x_name: &'a str,
    y_name: &'a str,
) -> Bucketer<'a, (T, (Float, u64))> {
    Bucketer {
        bucketing_function: &|(_, (x, y))| usize::exact_from(max(x.complexity(), *y)),
        bucketing_label: format!("max({x_name}.complexity(), {y_name})"),
    }
}
