use integer::Integer;
use natural::Natural;
use std::ops::Not;

/// Interpreting a slice of `u32`s, `limbs_in`, as the limbs (in ascending order) of a `Natural`,
/// takes the bitwise not of the `Natural` and writes its limbs to the lowest `limbs_in.len()` limbs
/// of `limbs_out`. For this to work, `limbs_out` must be at least as long as `limbs_in`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs_in.len()`
///
/// # Panics
/// Panics if `limbs_out` is shorter than `limbs_in`.
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::not::limbs_not;
///
/// let mut limbs_out = [0, 1, 2];
/// limbs_not(&mut limbs_out, &[0xffff0000, 0xf0f0f0f0]);
/// assert_eq!(limbs_out, [0x0000ffff, 0x0f0f0f0f, 2]);
/// ```
pub fn limbs_not(limbs_out: &mut [u32], limbs_in: &[u32]) {
    let n = limbs_in.len();
    assert!(limbs_out.len() >= n);
    for i in 0..n {
        limbs_out[i] = !limbs_in[i];
    }
}

/// Interpreting a slice of `u32`s, as the limbs (in ascending order) of a `Natural`, takes the
/// bitwise not of the `Natural` in place.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::logic::not::limbs_not_in_place;
/// use std::cmp::Ordering;
///
/// let mut limbs = [0, 1, 2];
/// limbs_not_in_place(&mut limbs);
/// assert_eq!(limbs, [0xffffffff, 0xfffffffe, 0xfffffffd]);
/// ```
pub fn limbs_not_in_place(limbs: &mut [u32]) {
    for limb in limbs.iter_mut() {
        *limb = !*limb;
    }
}

/// Returns the bitwise complement of a `Natural`, as if it were represented in two's complement,
/// taking the `Natural` by value and returning an `Integer`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((!Natural::ZERO).to_string(), "-1");
///     assert_eq!((!Natural::from(123u32)).to_string(), "-124");
/// }
/// ```
impl Not for Natural {
    type Output = Integer;

    fn not(self) -> Integer {
        Integer {
            sign: false,
            abs: self + 1,
        }
    }
}

/// Returns the bitwise complement of a `Natural`, as if it were represented in two's complement,
/// taking the `Natural` by reference and returning an `Integer`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((!&Natural::ZERO).to_string(), "-1");
///     assert_eq!((!&Natural::from(123u32)).to_string(), "-124");
/// }
/// ```
impl<'a> Not for &'a Natural {
    type Output = Integer;

    fn not(self) -> Integer {
        Integer {
            sign: false,
            abs: self + 1,
        }
    }
}
