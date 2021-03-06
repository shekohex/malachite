use itertools::Itertools;
use malachite_base::num::arithmetic::traits::{DivRound, Parity, ShrRound};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::string::to_string::{
    digit_to_display_byte_lower, digit_to_display_byte_upper,
};
use malachite_base::num::conversion::string::BaseFmtWrapper as BaseBaseFmtWrapper;
use malachite_base::num::conversion::traits::{
    Digits, ExactFrom, PowerOfTwoDigitIterable, ToStringBase, WrappingFrom,
};
use malachite_base::num::logic::traits::{BitIterable, SignificantBits};
use malachite_base::rounding_modes::RoundingMode;
use natural::conversion::digits::general_digits::{_limbs_to_digits_small_base, limbs_digit_count};
use natural::conversion::string::BaseFmtWrapper;
use natural::logic::significant_bits::limbs_significant_bits;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;
use std::fmt::{Binary, Debug, Display, Formatter, LowerHex, Octal, Result, UpperHex, Write};

impl<'a> Display for BaseFmtWrapper<&'a Natural> {
    /// Writes a wrapped `Natural` to a string using a specified base.
    ///
    /// If the base is greater than 10, lowercase alphabetic letters are used by default. Using the
    /// `#` flag switches to uppercase letters. Padding with zeros works as usual.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 36.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::conversion::string::BaseFmtWrapper;
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = Natural::from(1000000000u32);
    /// let x = BaseFmtWrapper::new(&n, 36);
    /// assert_eq!(format!("{}", x), "gjdgxs");
    /// assert_eq!(format!("{:#}", x), "GJDGXS");
    /// assert_eq!(format!("{:010}", x), "0000gjdgxs");
    /// assert_eq!(format!("{:#010}", x), "0000GJDGXS");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        assert!((2..=36).contains(&self.base), "base out of range");
        if let Natural(Small(x)) = self.x {
            Display::fmt(&BaseBaseFmtWrapper::new(*x, self.base), f)
        } else {
            let mut digits = self.x.to_digits_desc(&u8::wrapping_from(self.base));
            if f.alternate() {
                for digit in &mut digits {
                    *digit = digit_to_display_byte_upper(*digit);
                }
            } else {
                for digit in &mut digits {
                    *digit = digit_to_display_byte_lower(*digit);
                }
            }
            f.pad_integral(true, "", std::str::from_utf8(&digits).unwrap())
        }
    }
}

impl<'a> Debug for BaseFmtWrapper<&'a Natural> {
    /// Writes a wrapped `Natural` to a string using a specified base.
    ///
    /// If the base is greater than 10, lowercase alphabetic letters are used by default. Using the
    /// `#` flag switches to uppercase letters. Padding with zeros works as usual.
    ///
    /// This is the same as the `Display::fmt` implementation.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 36.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::conversion::string::BaseFmtWrapper;
    /// use malachite_nz::natural::Natural;
    ///
    /// let n = Natural::from(1000000000u32);
    /// let x = BaseFmtWrapper::new(&n, 36);
    /// assert_eq!(format!("{:?}", x), "gjdgxs");
    /// assert_eq!(format!("{:#?}", x), "GJDGXS");
    /// assert_eq!(format!("{:010?}", x), "0000gjdgxs");
    /// assert_eq!(format!("{:#010?}", x), "0000GJDGXS");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}

impl ToStringBase for Natural {
    /// Converts a `Natural` to a string using a specified base.
    ///
    /// Digits from 0 to 9 become `char`s from '0' to '9'. Digits from 10 to 35 become the lowercase
    /// `char`s 'a' to 'z'.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 36.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::conversion::traits::ToStringBase;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(1000u32).to_string_base(2), "1111101000");
    /// assert_eq!(Natural::from(1000u32).to_string_base(10), "1000");
    /// assert_eq!(Natural::from(1000u32).to_string_base(36), "rs");
    /// ```
    fn to_string_base(&self, base: u64) -> String {
        assert!((2..=36).contains(&base), "base out of range");
        if let Natural(Small(x)) = self {
            x.to_string_base(base)
        } else {
            let mut digits = self.to_digits_desc(&u8::wrapping_from(base));
            for digit in &mut digits {
                *digit = digit_to_display_byte_lower(*digit);
            }
            String::from_utf8(digits).unwrap()
        }
    }

    /// Converts a `Natural` to a string using a specified base.
    ///
    /// Digits from 0 to 9 become `char`s from '0' to '9'. Digits from 10 to 35 become the uppercase
    /// `char`s 'A' to 'Z'.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or greater than 36.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::conversion::traits::ToStringBase;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(1000u32).to_string_base_upper(2), "1111101000");
    /// assert_eq!(Natural::from(1000u32).to_string_base_upper(10), "1000");
    /// assert_eq!(Natural::from(1000u32).to_string_base_upper(36), "RS");
    /// ```
    fn to_string_base_upper(&self, base: u64) -> String {
        assert!((2..=36).contains(&base), "base out of range");
        if let Natural(Small(x)) = self {
            x.to_string_base_upper(base)
        } else {
            let mut digits = self.to_digits_desc(&u8::wrapping_from(base));
            for digit in &mut digits {
                *digit = digit_to_display_byte_upper(*digit);
            }
            String::from_utf8(digits).unwrap()
        }
    }
}

impl Display for Natural {
    /// Converts a `Natural` to a `String`.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::ZERO.to_string(), "0");
    /// assert_eq!(Natural::from(123u32).to_string(), "123");
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().to_string(), "1000000000000");
    /// assert_eq!(format!("{:05}", Natural::from(123u32)), "00123");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Natural(Small(x)) => Display::fmt(x, f),
            Natural(Large(xs)) => {
                let mut digits = vec![0; usize::exact_from(limbs_digit_count(xs, 10))];
                let mut xs = xs.clone();
                let len = _limbs_to_digits_small_base(&mut digits, 10, &mut xs, None);
                digits.truncate(len);
                for digit in &mut digits {
                    *digit = digit_to_display_byte_lower(*digit);
                }
                f.pad_integral(true, "", std::str::from_utf8(&digits).unwrap())
            }
        }
    }
}

impl Debug for Natural {
    /// Converts a `Natural` to a `String`.
    ///
    /// This is the same as the `Display::fmt` implementation.
    ///
    /// # Worst-case complexity
    /// TODO
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::ZERO.to_debug_string(), "0");
    /// assert_eq!(Natural::from(123u32).to_debug_string(), "123");
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().to_debug_string(), "1000000000000");
    /// assert_eq!(format!("{:05?}", Natural::from(123u32)), "00123");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}

#[doc(hidden)]
pub struct NaturalAlt(pub Natural);

#[doc(hidden)]
pub struct NaturalAlt2(pub Natural);

impl Binary for NaturalAlt {
    #[doc(hidden)]
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Natural(Small(x)) = self.0 {
            Binary::fmt(&x, f)
        } else {
            if f.alternate() {
                f.write_str("0b")?;
            }
            if let Some(width) = f.width() {
                let mut len = usize::exact_from(self.0.significant_bits());
                if f.alternate() {
                    len += 2;
                }
                for _ in 0..width.saturating_sub(len) {
                    f.write_char('0')?;
                }
            }
            for bit in self.0.bits().rev() {
                f.write_char(if bit { '1' } else { '0' })?;
            }
            Ok(())
        }
    }
}

impl Binary for NaturalAlt2 {
    #[doc(hidden)]
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self.0 {
            Natural(Small(x)) => Binary::fmt(x, f),
            Natural(Large(ref xs)) => {
                let (xs_last, xs_init) = xs.split_last().unwrap();
                let width = if let Some(width) = f.width() {
                    width.saturating_sub(xs_init.len() << Limb::LOG_WIDTH)
                } else {
                    0
                };
                let mut result = if f.alternate() {
                    write!(f, "{:#0width$b}", xs_last, width = width)
                } else {
                    write!(f, "{:0width$b}", xs_last, width = width)
                };
                for x in xs_init.iter().rev() {
                    #[cfg(feature = "32_bit_limbs")]
                    {
                        result = write!(f, "{:032b}", x);
                    }
                    #[cfg(not(feature = "32_bit_limbs"))]
                    {
                        result = write!(f, "{:064b}", x);
                    }
                }
                result
            }
        }
    }
}

impl Binary for Natural {
    /// Converts a `Natural` to a binary `String`.
    ///
    /// Using the `#` format flag prepends `"0b"` to the string.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::strings::ToBinaryString;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::ZERO.to_binary_string(), "0");
    /// assert_eq!(Natural::from(123u32).to_binary_string(), "1111011");
    /// assert_eq!(
    ///     Natural::from_str("1000000000000").unwrap().to_binary_string(),
    ///     "1110100011010100101001010001000000000000"
    /// );
    /// assert_eq!(format!("{:011b}", Natural::from(123u32)), "00001111011");
    ///
    /// assert_eq!(format!("{:#b}", Natural::ZERO), "0b0");
    /// assert_eq!(format!("{:#b}", Natural::from(123u32)), "0b1111011");
    /// assert_eq!(
    ///     format!("{:#b}", Natural::from_str("1000000000000").unwrap()),
    ///     "0b1110100011010100101001010001000000000000"
    /// );
    /// assert_eq!(format!("{:#011b}", Natural::from(123u32)), "0b001111011");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Natural(Small(x)) => Binary::fmt(x, f),
            Natural(Large(xs)) => {
                let mut bits = vec![0; usize::exact_from(limbs_significant_bits(xs))];
                let mut limbs = xs.iter();
                let mut limb = *limbs.next().unwrap();
                let mut remaining_bits = Limb::WIDTH;
                for bit in bits.iter_mut().rev() {
                    if remaining_bits == 0 {
                        remaining_bits = Limb::WIDTH;
                        limb = *limbs.next().unwrap();
                    }
                    *bit = if limb.even() { b'0' } else { b'1' };
                    limb >>= 1;
                    remaining_bits -= 1;
                }
                f.pad_integral(true, "0b", std::str::from_utf8(&bits).unwrap())
            }
        }
    }
}

impl Octal for NaturalAlt {
    #[doc(hidden)]
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Natural(Small(x)) = self.0 {
            Octal::fmt(&x, f)
        } else {
            if f.alternate() {
                f.write_str("0o")?;
            }
            if let Some(width) = f.width() {
                let mut len = usize::exact_from(
                    self.0
                        .significant_bits()
                        .div_round(3, RoundingMode::Ceiling),
                );
                if f.alternate() {
                    len += 2;
                }
                for _ in 0..width.saturating_sub(len) {
                    f.write_char('0')?;
                }
            }
            for digit in PowerOfTwoDigitIterable::<u8>::power_of_two_digits(&self.0, 3).rev() {
                f.write_char(char::from(digit_to_display_byte_lower(digit)))?;
            }
            Ok(())
        }
    }
}

#[cfg(feature = "32_bit_limbs")]
fn oz_fmt(f: &mut Formatter, x: Limb) -> Result {
    write!(f, "{:08o}", x)
}
#[cfg(not(feature = "32_bit_limbs"))]
fn oz_fmt(f: &mut Formatter, x: Limb) -> Result {
    write!(f, "{:016o}", x)
}

impl Octal for NaturalAlt2 {
    #[doc(hidden)]
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self.0 {
            Natural(Small(x)) => Octal::fmt(x, f),
            Natural(Large(xs)) => {
                if f.alternate() {
                    f.write_str("0o")?;
                }
                if let Some(width) = f.width() {
                    let mut len = usize::exact_from(
                        limbs_significant_bits(xs).div_round(3, RoundingMode::Ceiling),
                    );
                    if f.alternate() {
                        len += 2;
                    }
                    for _ in 0..width.saturating_sub(len) {
                        f.write_char('0')?;
                    }
                }
                let mut triple_r = xs.len() % 3;
                if triple_r == 0 {
                    triple_r = 3;
                }
                let mut result;
                let last_i = xs.len() - 1;
                const W_1_2: u64 = Limb::WIDTH >> 1;
                const W_1_4: u64 = Limb::WIDTH >> 2;
                const W_3_4: u64 = W_1_4 * 3;
                const MASK: Limb = (1 << W_3_4) - 1;
                match triple_r {
                    1 => {
                        let x_2 = xs[last_i];
                        let y = x_2 >> W_3_4;
                        if y == 0 {
                            result = write!(f, "{:o}", x_2 & MASK);
                        } else {
                            write!(f, "{:o}", y).unwrap();
                            result = oz_fmt(f, x_2 & MASK);
                        }
                    }
                    2 => {
                        let x_1 = xs[last_i];
                        let x_2 = xs[last_i - 1];
                        let y = x_1 >> W_1_2;
                        if y == 0 {
                            write!(f, "{:o}", ((x_1 << W_1_4) & MASK) | (x_2 >> W_3_4)).unwrap();
                        } else {
                            write!(f, "{:o}", y).unwrap();
                            oz_fmt(f, ((x_1 << W_1_4) & MASK) | (x_2 >> W_3_4)).unwrap();
                        }
                        result = oz_fmt(f, x_2 & MASK);
                    }
                    _ => {
                        let x_0 = xs[last_i];
                        let x_1 = xs[last_i - 1];
                        let x_2 = xs[last_i - 2];
                        let y = x_0 >> W_1_4;
                        if y == 0 {
                            write!(f, "{:o}", ((x_0 << W_1_2) & MASK) | (x_1 >> W_1_2)).unwrap();
                        } else {
                            write!(f, "{:o}", y).unwrap();
                            oz_fmt(f, ((x_0 << W_1_2) & MASK) | (x_1 >> W_1_2)).unwrap();
                        }
                        oz_fmt(f, ((x_1 << W_1_4) & MASK) | (x_2 >> W_3_4)).unwrap();
                        result = oz_fmt(f, x_2 & MASK);
                    }
                }
                for mut chunk in &xs.iter().rev().skip(triple_r).chunks(3) {
                    let x_0 = chunk.next().unwrap();
                    let x_1 = chunk.next().unwrap();
                    let x_2 = chunk.next().unwrap();
                    oz_fmt(f, x_0 >> W_1_4).unwrap();
                    oz_fmt(f, ((x_0 << W_1_2) & MASK) | (x_1 >> W_1_2)).unwrap();
                    oz_fmt(f, ((x_1 << W_1_4) & MASK) | (x_2 >> W_3_4)).unwrap();
                    result = oz_fmt(f, x_2 & MASK);
                }
                result
            }
        }
    }
}

impl Octal for Natural {
    /// Converts a `Natural` to an octal `String`.
    ///
    /// Using the `#` format flag prepends `"0o"` to the string.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::strings::ToOctalString;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::ZERO.to_octal_string(), "0");
    /// assert_eq!(Natural::from(123u32).to_octal_string(), "173");
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().to_octal_string(), "16432451210000");
    /// assert_eq!(format!("{:07o}", Natural::from(123u32)), "0000173");
    ///
    /// assert_eq!(format!("{:#o}", Natural::ZERO), "0o0");
    /// assert_eq!(format!("{:#o}", Natural::from(123u32)), "0o173");
    /// assert_eq!(
    ///     format!("{:#o}", Natural::from_str("1000000000000").unwrap()),
    ///     "0o16432451210000"
    /// );
    /// assert_eq!(format!("{:#07o}", Natural::from(123u32)), "0o00173");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Natural(Small(x)) => Octal::fmt(x, f),
            Natural(Large(xs)) => {
                let mut digits = vec![
                    0;
                    usize::exact_from(
                        limbs_significant_bits(xs).div_round(3, RoundingMode::Ceiling)
                    )
                ];
                let mut limbs = xs.iter();
                let mut remaining_bits = Limb::WIDTH;
                let mut limb = *limbs.next().unwrap();
                for digit in digits.iter_mut().rev() {
                    if remaining_bits >= 3 {
                        *digit = digit_to_display_byte_lower(u8::wrapping_from(limb & 7));
                        remaining_bits -= 3;
                        limb >>= 3;
                    } else {
                        match remaining_bits {
                            0 => {
                                limb = *limbs.next().unwrap();
                                *digit = digit_to_display_byte_lower(u8::wrapping_from(limb & 7));
                                remaining_bits = Limb::WIDTH - 3;
                                limb >>= 3;
                            }
                            1 => {
                                let previous_limb = limb;
                                limb = *limbs.next().unwrap_or(&0);
                                *digit = digit_to_display_byte_lower(u8::wrapping_from(
                                    ((limb & 3) << 1) | previous_limb,
                                ));
                                remaining_bits = Limb::WIDTH - 2;
                                limb >>= 2;
                            }
                            _ => {
                                let previous_limb = limb;
                                limb = *limbs.next().unwrap_or(&0);
                                *digit = digit_to_display_byte_lower(u8::wrapping_from(
                                    ((limb & 1) << 2) | previous_limb,
                                ));
                                remaining_bits = Limb::WIDTH - 1;
                                limb >>= 1;
                            }
                        }
                    }
                }
                f.pad_integral(true, "0o", std::str::from_utf8(&digits).unwrap())
            }
        }
    }
}

impl LowerHex for NaturalAlt {
    #[doc(hidden)]
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Natural(Small(x)) = self.0 {
            LowerHex::fmt(&x, f)
        } else {
            if f.alternate() {
                f.write_str("0x")?;
            }
            if let Some(width) = f.width() {
                let mut len = usize::exact_from(
                    self.0
                        .significant_bits()
                        .shr_round(2, RoundingMode::Ceiling),
                );
                if f.alternate() {
                    len += 2;
                }
                for _ in 0..width.saturating_sub(len) {
                    f.write_char('0')?;
                }
            }
            for digit in PowerOfTwoDigitIterable::<u8>::power_of_two_digits(&self.0, 4).rev() {
                f.write_char(char::from(digit_to_display_byte_lower(digit)))?;
            }
            Ok(())
        }
    }
}

impl LowerHex for NaturalAlt2 {
    #[doc(hidden)]
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self.0 {
            Natural(Small(x)) => LowerHex::fmt(x, f),
            Natural(Large(ref xs)) => {
                let (xs_last, xs_init) = xs.split_last().unwrap();
                let width = if let Some(width) = f.width() {
                    width.saturating_sub(xs_init.len() << Limb::LOG_WIDTH >> 2)
                } else {
                    0
                };
                let mut result = if f.alternate() {
                    write!(f, "{:#0width$x}", xs_last, width = width)
                } else {
                    write!(f, "{:0width$x}", xs_last, width = width)
                };
                for x in xs_init.iter().rev() {
                    #[cfg(feature = "32_bit_limbs")]
                    {
                        result = write!(f, "{:08x}", x);
                    }
                    #[cfg(not(feature = "32_bit_limbs"))]
                    {
                        result = write!(f, "{:016x}", x);
                    }
                }
                result
            }
        }
    }
}

impl LowerHex for Natural {
    /// Converts a `Natural` to a hexadecimal `String` using lowercase characters.
    ///
    /// Using the `#` format flag prepends `"0x"` to the string.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::strings::ToLowerHexString;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::ZERO.to_lower_hex_string(), "0");
    /// assert_eq!(Natural::from(123u32).to_lower_hex_string(), "7b");
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().to_lower_hex_string(), "e8d4a51000");
    /// assert_eq!(format!("{:07x}", Natural::from(123u32)), "000007b");
    ///
    /// assert_eq!(format!("{:#x}", Natural::ZERO), "0x0");
    /// assert_eq!(format!("{:#x}", Natural::from(123u32)), "0x7b");
    /// assert_eq!(format!("{:#x}", Natural::from_str("1000000000000").unwrap()), "0xe8d4a51000");
    /// assert_eq!(format!("{:#07x}", Natural::from(123u32)), "0x0007b");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Natural(Small(x)) => LowerHex::fmt(x, f),
            Natural(Large(xs)) => {
                const DIGITS_PER_LIMB: u64 = Limb::WIDTH >> 2;
                let mut digits = vec![
                    0;
                    usize::exact_from(
                        limbs_significant_bits(xs).shr_round(2, RoundingMode::Ceiling)
                    )
                ];
                let mut limbs = xs.iter();
                let mut limb = *limbs.next().unwrap();
                let mut remaining_digits = DIGITS_PER_LIMB;
                for digit in digits.iter_mut().rev() {
                    if remaining_digits == 0 {
                        remaining_digits = DIGITS_PER_LIMB;
                        limb = *limbs.next().unwrap();
                    }
                    *digit = digit_to_display_byte_lower(u8::wrapping_from(limb & 15));
                    limb >>= 4;
                    remaining_digits -= 1;
                }
                f.pad_integral(true, "0x", std::str::from_utf8(&digits).unwrap())
            }
        }
    }
}

impl UpperHex for Natural {
    /// Converts a `Natural` to a hexadecimal `String` using uppercase characters.
    ///
    /// Using the `#` format flag prepends `"0x"` to the string.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::strings::ToUpperHexString;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::ZERO.to_upper_hex_string(), "0");
    /// assert_eq!(Natural::from(123u32).to_upper_hex_string(), "7B");
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().to_upper_hex_string(), "E8D4A51000");
    /// assert_eq!(format!("{:07X}", Natural::from(123u32)), "000007B");
    ///
    /// assert_eq!(format!("{:#X}", Natural::ZERO), "0x0");
    /// assert_eq!(format!("{:#X}", Natural::from(123u32)), "0x7B");
    /// assert_eq!(format!("{:#X}", Natural::from_str("1000000000000").unwrap()), "0xE8D4A51000");
    /// assert_eq!(format!("{:#07X}", Natural::from(123u32)), "0x0007B");
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Natural(Small(x)) => UpperHex::fmt(x, f),
            Natural(Large(xs)) => {
                const DIGITS_PER_LIMB: u64 = Limb::WIDTH >> 2;
                let mut digits = vec![
                    0;
                    usize::exact_from(
                        limbs_significant_bits(xs).shr_round(2, RoundingMode::Ceiling)
                    )
                ];
                let mut limbs = xs.iter();
                let mut limb = *limbs.next().unwrap();
                let mut remaining_digits = DIGITS_PER_LIMB;
                for digit in digits.iter_mut().rev() {
                    if remaining_digits == 0 {
                        remaining_digits = DIGITS_PER_LIMB;
                        limb = *limbs.next().unwrap();
                    }
                    *digit = digit_to_display_byte_upper(u8::wrapping_from(limb & 15));
                    limb >>= 4;
                    remaining_digits -= 1;
                }
                f.pad_integral(true, "0x", std::str::from_utf8(&digits).unwrap())
            }
        }
    }
}
