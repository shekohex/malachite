/// Functions for producing iterators over the digits of a [`Rational`](crate::Rational).
#[allow(clippy::module_inception)]
pub mod digits;
/// Functions for constructing a [`Rational`](crate::Rational) from digits.
pub mod from_digits;
/// Functions for constructing a [`Rational`](crate::Rational) from base-$2^k$ digits.
pub mod from_power_of_2_digits;
/// Functions for producing iterators over the base-$2^k$ digits of a
/// [`Rational`](crate::Rational).
pub mod power_of_2_digits;
/// Functions for returning the digits of a [`Rational`](crate::Rational). The digits after the
/// point are returned as a
/// [`RationalSequence`](malachite_base::rational_sequences::RationalSequence).
pub mod to_digits;
/// Functions for returning the base-$2^k$ digits of a [`Rational`](crate::Rational). The digits
/// after the point are returned as a
/// [`RationalSequence`](malachite_base::rational_sequences::RationalSequence).
pub mod to_power_of_2_digits;
