use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_nz::natural::Natural;
use std::cmp::Ordering;

fn float_partial_cmp_unsigned<T: PrimitiveUnsigned>(x: &Float, y: &T) -> Option<Ordering>
where
    Natural: From<T>,
{
    match (x, y) {
        (float_nan!(), _) => None,
        (float_infinity!(), _) => Some(Ordering::Greater),
        (float_negative_infinity!(), _) => Some(Ordering::Less),
        (float_either_zero!(), _) => Some(if *y == T::ZERO {
            Ordering::Equal
        } else {
            Ordering::Less
        }),
        (
            Float(Finite {
                sign: s_x,
                exponent: e_x,
                significand: sig_x,
                ..
            }),
            y,
        ) => Some(if !s_x {
            Ordering::Less
        } else if *y == T::ZERO {
            Ordering::Greater
        } else if *e_x <= 0 {
            Ordering::Less
        } else {
            e_x.unsigned_abs()
                .cmp(&y.significant_bits())
                .then_with(|| sig_x.cmp_normalized(&Natural::from(*y)))
        }),
    }
}

macro_rules! impl_from_unsigned {
    ($t: ident) => {
        impl PartialOrd<$t> for Float {
            /// Compares a [`Float`] to an unsigned primitive integer.
            ///
            /// NaN is not comparable to any primitive integer. Infinity is greater than any
            /// primitive integer, and negative infinity is less. Both the [`Float`] zero and the
            /// [`Float`] negative zero are equal to the integer zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                float_partial_cmp_unsigned(self, other)
            }
        }

        impl PartialOrd<Float> for $t {
            /// Compares an unsigned primitive integer to a [`Float`].
            ///
            /// No integer is comparable to NaN. Every integer is smaller than infinity and greater
            /// than negative infinity. The integer zero is equal to both the [`Float`] zero and
            /// the [`Float`] negative zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &Float) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_unsigneds!(impl_from_unsigned);

fn float_partial_cmp_signed<T: PrimitiveSigned>(x: &Float, y: &T) -> Option<Ordering>
where
    Natural: From<<T as UnsignedAbs>::Output>,
{
    match (x, y) {
        (float_nan!(), _) => None,
        (float_infinity!(), _) => Some(Ordering::Greater),
        (float_negative_infinity!(), _) => Some(Ordering::Less),
        (float_either_zero!(), _) => Some(T::ZERO.cmp(y)),
        (
            Float(Finite {
                sign: s_x,
                exponent: e_x,
                significand: sig_x,
                ..
            }),
            y,
        ) => {
            let s_y = *y > T::ZERO;
            let s_cmp = s_x.cmp(&s_y);
            if s_cmp != Ordering::Equal {
                return Some(s_cmp);
            }
            let abs_cmp = if *y == T::ZERO {
                Ordering::Greater
            } else if *e_x <= 0 {
                Ordering::Less
            } else {
                e_x.unsigned_abs()
                    .cmp(&y.significant_bits())
                    .then_with(|| sig_x.cmp_normalized(&Natural::from(y.unsigned_abs())))
            };
            Some(if s_y { abs_cmp } else { abs_cmp.reverse() })
        }
    }
}

macro_rules! impl_from_signed {
    ($t: ident) => {
        impl PartialOrd<$t> for Float {
            /// Compares a [`Float`] to a signed primitive integer.
            ///
            /// NaN is not comparable to any primitive integer. Infinity is greater than any
            /// primitive integer, and negative infinity is less. Both the [`Float`] zero and the
            /// [`Float`] negative zero are equal to the integer zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                float_partial_cmp_signed(self, other)
            }
        }

        impl PartialOrd<Float> for $t {
            /// Compares a signed primitive integer to a [`Float`].
            ///
            /// No integer is comparable to NaN. Every integer is smaller than infinity and greater
            /// than negative infinity. The integer zero is equal to both the [`Float`] zero and
            /// the [`Float`] negative zero.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `other.significant_bits()`.
            ///
            /// # Examples
            /// See [here](super::partial_cmp_primitive_int#partial_cmp).
            #[inline]
            fn partial_cmp(&self, other: &Float) -> Option<Ordering> {
                other.partial_cmp(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_signeds!(impl_from_signed);
