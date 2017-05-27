use natural::Natural::{self, Large, Small};
use std::mem::swap;
use std::ops::{Add, AddAssign};

/// Adds a `Natural` to a `Natural`, taking ownership of both `Natural`s.
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((Natural::from(0u32) + Natural::from(123u32)).to_string(), "123");
/// assert_eq!((Natural::from(123u32) + Natural::from(0u32)).to_string(), "123");
/// assert_eq!((Natural::from(123u32) + Natural::from(456u32)).to_string(), "579");
/// assert_eq!((Natural::from_str("1000000000000").unwrap() + Natural::from_str("2000000000000")
///            .unwrap()).to_string(), "3000000000000");
/// ```
impl Add<Natural> for Natural {
    type Output = Natural;

    fn add(mut self, other: Natural) -> Natural {
        self += other;
        self
    }
}

/// Adds a `Natural` to a `Natural` in place.
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// let mut x = Natural::new();
/// x += Natural::from_str("1000000000000").unwrap();
/// x += Natural::from_str("2000000000000").unwrap();
/// x += Natural::from_str("3000000000000").unwrap();
/// x += Natural::from_str("4000000000000").unwrap();
/// assert_eq!(x.to_string(), "10000000000000");
/// ```
impl AddAssign<Natural> for Natural {
    fn add_assign(&mut self, mut other: Natural) {
        if self.limb_count() < other.limb_count() {
            swap(self, &mut other);
        }
        match other {
            Small(y) => *self += y,
            Large(ref ys) => {
                match *self {
                    Large(ref mut xs) => large_add(xs, ys),
                    _ => unreachable!(),
                }
            }
        }
    }
}

fn add_and_carry(x: u32, y: u32, carry: &mut bool) -> u32 {
    let (sum, overflow) = x.overflowing_add(y);
    if *carry {
        *carry = overflow;
        let (sum, overflow) = sum.overflowing_add(1);
        *carry |= overflow;
        sum
    } else {
        *carry = overflow;
        sum
    }
}

// assumes that xs.len() >= ys.len()
fn large_add(xs: &mut Vec<u32>, ys: &[u32]) {
    let mut ys_iter = ys.iter();
    let mut carry = false;
    for x in xs.iter_mut() {
        match ys_iter.next() {
            Some(y) => *x = add_and_carry(*x, *y, &mut carry),
            None if carry => *x = add_and_carry(*x, 0, &mut carry),
            None => break,
        }
    }
    if carry {
        xs.push(1);
    }
}
