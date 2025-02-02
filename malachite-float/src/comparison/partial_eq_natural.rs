use crate::Float;
use crate::InnerFloat::{Finite, Zero};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::Natural;
use std::cmp::Ordering;

impl PartialEq<Natural> for Float {
    /// Determines whether a [`Float`] is equal to a [`Natural`].
    ///
    /// Infinity, negative infinity, and NaN are not equal to any [`Natural`]. Both the [`Float`]
    /// zero and the [`Float`] negative zero are equal to the [`Natural`] zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `min(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Float::from(123) == Natural::from(123u32));
    /// assert!(Float::ONE_HALF != Natural::ONE);
    /// ```
    fn eq(&self, other: &Natural) -> bool {
        match self {
            float_either_zero!() => *other == 0u32,
            Float(Finite {
                sign,
                exponent,
                significand,
                ..
            }) => {
                *sign
                    && *other != 0u32
                    && *exponent >= 0
                    && other.significant_bits() == exponent.unsigned_abs()
                    && significand.cmp_normalized(other) == Ordering::Equal
            }
            _ => false,
        }
    }
}

impl PartialEq<Float> for Natural {
    /// Determines whether a [`Natural`] is equal to a [`Float`].
    ///
    /// No [`Natural`] is equal to infinity, negative infinity, or NaN. The [`Natural`] zero is
    /// equal to both the [`Float`] zero and the [`Float`] negative zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `min(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::from(123u32) == Float::from(123));
    /// assert!(Natural::ONE != Float::ONE_HALF);
    /// ```
    #[inline]
    fn eq(&self, other: &Float) -> bool {
        other == self
    }
}
