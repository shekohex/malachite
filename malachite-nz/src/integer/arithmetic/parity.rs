use integer::Integer;
use malachite_base::num::arithmetic::traits::Parity;

impl<'a> Parity for &'a Integer {
    /// Tests whether an [`Integer`] is even.
    ///
    /// $f(x) = (2|x)$.
    ///
    /// $f(x) = (\exists k \in \N : x = 2k)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::{Parity, Pow};
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.even(), true);
    /// assert_eq!(Integer::from(123).even(), false);
    /// assert_eq!(Integer::from(-0x80).even(), true);
    /// assert_eq!(Integer::from(10u32).pow(12).even(), true);
    /// assert_eq!((-Integer::from(10u32).pow(12) - Integer::ONE).even(), false);
    /// ```
    fn even(self) -> bool {
        self.abs.even()
    }

    /// Tests whether an [`Integer`] is odd.
    ///
    /// $f(x) = (2\nmid x)$.
    ///
    /// $f(x) = (\exists k \in \N : x = 2k+1)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::{Parity, Pow};
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.odd(), false);
    /// assert_eq!(Integer::from(123).odd(), true);
    /// assert_eq!(Integer::from(-0x80).odd(), false);
    /// assert_eq!(Integer::from(10u32).pow(12).odd(), false);
    /// assert_eq!((-Integer::from(10u32).pow(12) - Integer::ONE).odd(), true);
    /// ```
    fn odd(self) -> bool {
        self.abs.odd()
    }
}
