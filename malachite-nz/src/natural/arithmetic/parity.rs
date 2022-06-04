use malachite_base::num::arithmetic::traits::Parity;
use natural::InnerNatural::{Large, Small};
use natural::Natural;

impl<'a> Parity for &'a Natural {
    /// Tests whether a [`Natural`] is even.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.even(), true);
    /// assert_eq!(Natural::from(123u32).even(), false);
    /// assert_eq!(Natural::from(0x80u32).even(), true);
    /// assert_eq!(Natural::from(10u32).pow(12).even(), true);
    /// assert_eq!((Natural::from(10u32).pow(12) + Natural::ONE).even(), false);
    /// ```
    fn even(self) -> bool {
        match self {
            Natural(Small(small)) => small.even(),
            Natural(Large(ref limbs)) => limbs[0].even(),
        }
    }

    /// Tests whether a [`Natural`] is odd.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.odd(), false);
    /// assert_eq!(Natural::from(123u32).odd(), true);
    /// assert_eq!(Natural::from(0x80u32).odd(), false);
    /// assert_eq!(Natural::from(10u32).pow(12).odd(), false);
    /// assert_eq!((Natural::from(10u32).pow(12) + Natural::ONE).odd(), true);
    /// ```
    fn odd(self) -> bool {
        match *self {
            Natural(Small(small)) => small.odd(),
            Natural(Large(ref limbs)) => limbs[0].odd(),
        }
    }
}
