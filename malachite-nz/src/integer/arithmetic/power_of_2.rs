use integer::Integer;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::One;

impl PowerOf2<u64> for Integer {
    /// Raises 2 to an integer power.
    ///
    /// $f(k) = 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::power_of_2(0), 1);
    /// assert_eq!(Integer::power_of_2(3), 8);
    /// assert_eq!(Integer::power_of_2(100).to_string(), "1267650600228229401496703205376");
    /// ```
    #[inline]
    fn power_of_2(pow: u64) -> Integer {
        Integer::ONE << pow
    }
}
