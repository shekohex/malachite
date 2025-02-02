use core::cmp::Ordering;

/// Determines equality between the absolute values of two numbers.
pub trait EqAbs<Rhs: ?Sized = Self> {
    /// Compares the absolute values of two numbers for equality, taking both by reference.
    fn eq_abs(&self, other: &Rhs) -> bool;

    /// Compares the absolute values of two numbers for inequality, taking both by reference.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of [`eq_abs`](Self::eq_abs).
    #[inline]
    fn ne_abs(&self, other: &Rhs) -> bool {
        !self.eq_abs(other)
    }
}

/// Determines equality between the absolute values of two numbers, where some pairs of numbers may
/// not be comparable.
pub trait PartialOrdAbs<Rhs: ?Sized = Self> {
    /// Compares the absolute values of two numbers, taking both by reference.
    ///
    /// If the two values are not comparable, `None` is returned.
    fn partial_cmp_abs(&self, other: &Rhs) -> Option<Ordering>;

    /// Determines whether the absolute value of one number is less than the absolute value of
    /// another.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of
    /// [`partial_cmp_abs`](Self::partial_cmp_abs).
    #[inline]
    fn lt_abs(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp_abs(other), Some(Ordering::Less))
    }

    /// Determines whether the absolute value of one number is less than or equal to the absolute
    /// value of another.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of
    /// [`partial_cmp_abs`](Self::partial_cmp_abs).
    #[inline]
    fn le_abs(&self, other: &Rhs) -> bool {
        matches!(
            self.partial_cmp_abs(other),
            Some(Ordering::Less) | Some(Ordering::Equal)
        )
    }

    /// Determines whether the absolute value of one number is greater than the absolute value of
    /// another.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of
    /// [`partial_cmp_abs`](Self::partial_cmp_abs).
    #[inline]
    fn gt_abs(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp_abs(other), Some(Ordering::Greater))
    }

    /// Determines whether the absolute value of one number is greater than or equal to the
    /// absolute value of another.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of
    /// [`partial_cmp_abs`](Self::partial_cmp_abs).
    #[inline]
    fn ge_abs(&self, other: &Rhs) -> bool {
        matches!(
            self.partial_cmp_abs(other),
            Some(Ordering::Greater) | Some(Ordering::Equal)
        )
    }
}

/// Compares the absolute values of two numbers.
pub trait OrdAbs: Eq + PartialOrdAbs<Self> {
    fn cmp_abs(&self, other: &Self) -> Ordering;
}
