use malachite_base::num::{AddMul, AddMulAssign, PrimitiveInteger, SplitInHalf};
use natural::arithmetic::add_limb::limbs_slice_add_limb_in_place;
use natural::Natural::{self, Large, Small};
use platform::{DoubleLimb, Limb};

// Multiply s1 and s2limb, and add the s1.len() least significant limbs of the product to r and
// write the result to r. Return the most significant limb of the product, plus carry-out from the
// addition. r.len() >= s1.len()
pub fn mpn_addmul_1(r: &mut [Limb], s1: &[Limb], s2limb: Limb) -> Limb {
    let s1_len = s1.len();
    assert!(r.len() >= s1_len);
    let mut carry = 0;
    let s2limb_double = DoubleLimb::from(s2limb);
    for i in 0..s1_len {
        let limb_result = DoubleLimb::from(r[i]) + DoubleLimb::from(s1[i]) * s2limb_double + carry;
        r[i] = limb_result.lower_half();
        carry = limb_result >> Limb::WIDTH;
    }
    carry as Limb
}

/// Adds the product of a `Natural` (b) and a `Limb` (c) to a `Natural` (self), taking `self` and b
/// by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `min(self.significant_bits(), b.significant_bits())`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMul;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::from(10u32).add_mul(Natural::from(3u32), 4), 22);
///     assert_eq!(Natural::trillion().add_mul(Natural::from(0x1_0000u32), 0x1_0000u32).to_string(),
///                "1004294967296");
/// }
/// ```
impl AddMul<Natural, Limb> for Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(mut self, b: Natural, c: Limb) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl AddMul<Natural, u32> for Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(self, b: Natural, c: u32) -> Natural {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of a `Natural` (b) and a `Limb` (c) to a `Natural` (self), taking `self` by
/// value and b by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `b.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMul;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(Natural::from(10u32).add_mul(&Natural::from(3u32), 4), 22);
///     assert_eq!(Natural::trillion().add_mul(&Natural::from(0x1_0000u32),
///         0x1_0000u32).to_string(), "1004294967296");
/// }
/// ```
impl<'a> AddMul<&'a Natural, Limb> for Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(mut self, b: &'a Natural, c: Limb) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> AddMul<&'a Natural, u32> for Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(self, b: &'a Natural, c: u32) -> Natural {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of a `Natural` (b) and a `Limb` (c) to a `Natural` (self), taking `self` by
/// reference and b by value.
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
/// use malachite_base::num::AddMul;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::from(10u32)).add_mul(Natural::from(3u32), 4), 22);
///     assert_eq!((&Natural::trillion()).add_mul(Natural::from(0x1_0000u32),
///         0x1_0000u32).to_string(), "1004294967296");
/// }
/// ```
impl<'a> AddMul<Natural, Limb> for &'a Natural {
    type Output = Natural;

    fn add_mul(self, b: Natural, c: Limb) -> Natural {
        if c == 0 || b == 0 as Limb {
            return self.clone();
        }
        if c == 1 {
            return self + b;
        }
        if let Small(small_b) = b {
            if let Some(product) = small_b.checked_mul(c) {
                return self + product;
            }
        }
        let mut result_limbs = self.to_limbs_asc();
        let a_len = result_limbs.len();
        let b_len = b.limb_count() as usize;
        if a_len < b_len {
            result_limbs.resize(b_len, 0);
        }
        let carry = match b {
            Small(small) => mpn_addmul_1(&mut result_limbs, &[small], c),
            Large(ref b_limbs) => mpn_addmul_1(&mut result_limbs, b_limbs, c),
        };
        if carry != 0 {
            if a_len > b_len {
                if limbs_slice_add_limb_in_place(&mut result_limbs[b_len..], carry) {
                    result_limbs.push(1);
                }
            } else {
                result_limbs.push(carry);
            }
        }
        Large(result_limbs)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> AddMul<Natural, u32> for &'a Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(self, b: Natural, c: u32) -> Natural {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of a `Natural` (b) and a `Limb` (c) to a `Natural` (self), taking `self` and b
/// by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `max(self.significant_bits(), b.significant_bits())`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMul;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::from(10u32)).add_mul(&Natural::from(3u32), 4), 22);
///     assert_eq!((&Natural::trillion()).add_mul(&Natural::from(0x1_0000u32),
///         0x1_0000u32).to_string(), "1004294967296");
/// }
/// ```
impl<'a, 'b> AddMul<&'a Natural, Limb> for &'b Natural {
    type Output = Natural;

    fn add_mul(self, b: &'a Natural, c: Limb) -> Natural {
        if c == 0 || *b == 0 as Limb {
            return self.clone();
        }
        if c == 1 {
            return self + b;
        }
        if let Small(small_b) = *b {
            if let Some(product) = small_b.checked_mul(c) {
                return self + product;
            }
        }
        let mut result_limbs = self.to_limbs_asc();
        let a_len = result_limbs.len();
        let b_len = b.limb_count() as usize;
        if a_len < b_len {
            result_limbs.resize(b_len, 0);
        }
        let carry = match *b {
            Small(small) => mpn_addmul_1(&mut result_limbs, &[small], c),
            Large(ref b_limbs) => mpn_addmul_1(&mut result_limbs, b_limbs, c),
        };
        if carry != 0 {
            if a_len > b_len {
                if limbs_slice_add_limb_in_place(&mut result_limbs[b_len..], carry) {
                    result_limbs.push(1);
                }
            } else {
                result_limbs.push(carry);
            }
        }
        Large(result_limbs)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a, 'b> AddMul<&'a Natural, u32> for &'b Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(self, b: &'a Natural, c: u32) -> Natural {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of a `Natural` (b) and a `Limb` (c) to a `Natural` (self), in place, taking b
/// by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `min(self.significant_bits(), b.significant_bits())`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMulAssign;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(10u32);
///     x.add_mul_assign(Natural::from(3u32), 4);
///     assert_eq!(x, 22);
///
///     let mut x = Natural::trillion();
///     x.add_mul_assign(Natural::from(0x1_0000u32), 0x1_0000u32);
///     assert_eq!(x.to_string(), "1004294967296");
/// }
/// ```
impl AddMulAssign<Natural, Limb> for Natural {
    fn add_mul_assign(&mut self, b: Natural, c: Limb) {
        if c == 0 || b == 0 as Limb {
            return;
        }
        if c == 1 {
            *self += b;
            return;
        }
        if let Small(small_b) = b {
            if let Some(product) = small_b.checked_mul(c) {
                *self += product;
                return;
            }
        }
        {
            let self_limbs = self.promote_in_place();
            let a_len = self_limbs.len();
            let b_len = b.limb_count() as usize;
            if a_len < b_len {
                self_limbs.resize(b_len, 0);
            }
            let carry = match b {
                Small(small) => mpn_addmul_1(self_limbs, &[small], c),
                Large(ref b_limbs) => mpn_addmul_1(self_limbs, b_limbs, c),
            };
            if carry != 0 {
                if a_len > b_len {
                    if limbs_slice_add_limb_in_place(&mut self_limbs[b_len..], carry) {
                        self_limbs.push(1);
                    }
                } else {
                    self_limbs.push(carry);
                }
            }
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl AddMulAssign<Natural, u32> for Natural {
    #[inline]
    fn add_mul_assign(&mut self, b: Natural, c: u32) {
        self.add_mul_assign(b, Limb::from(c));
    }
}

/// Adds the product of a `Natural` (b) and a `Limb` (c) to a `Natural` (self), in place, taking b
/// by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `b.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::AddMulAssign;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(10u32);
///     x.add_mul_assign(&Natural::from(3u32), 4);
///     assert_eq!(x, 22);
///
///     let mut x = Natural::trillion();
///     x.add_mul_assign(&Natural::from(0x1_0000u32), 0x1_0000u32);
///     assert_eq!(x.to_string(), "1004294967296");
/// }
/// ```
impl<'a> AddMulAssign<&'a Natural, Limb> for Natural {
    fn add_mul_assign(&mut self, b: &'a Natural, c: Limb) {
        if c == 0 || *b == 0 as Limb {
            return;
        }
        if c == 1 {
            *self += b;
            return;
        }
        if let Small(small_b) = *b {
            if let Some(product) = small_b.checked_mul(c) {
                *self += product;
                return;
            }
        }
        {
            let self_limbs = self.promote_in_place();
            let a_len = self_limbs.len();
            let b_len = b.limb_count() as usize;
            if a_len < b_len {
                self_limbs.resize(b_len, 0);
            }
            let carry = match *b {
                Small(small) => mpn_addmul_1(self_limbs, &[small], c),
                Large(ref b_limbs) => mpn_addmul_1(self_limbs, b_limbs, c),
            };
            if carry != 0 {
                if a_len > b_len {
                    if limbs_slice_add_limb_in_place(&mut self_limbs[b_len..], carry) {
                        self_limbs.push(1);
                    }
                } else {
                    self_limbs.push(carry);
                }
            }
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> AddMulAssign<&'a Natural, u32> for Natural {
    #[inline]
    fn add_mul_assign(&mut self, b: &'a Natural, c: u32) {
        self.add_mul_assign(b, Limb::from(c));
    }
}
