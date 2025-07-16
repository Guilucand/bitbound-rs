use std::ops::{BitAnd, Shl, Sub};

use crate::const_assertions::{AssertLessEq, AssertPowerOfTwo};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct BoundedNumber<T, const UPPER_BOUND: usize>(T);

impl<T, const UPPER_BOUND: usize> BoundedNumber<T, UPPER_BOUND> {
    pub const unsafe fn new_unchecked(value: T) -> Self {
        Self(value)
    }
}
impl<
    T: num_traits::ConstOne + BitAnd<Output = T> + Sub<Output = T> + Shl<usize, Output = T>,
    const UPPER_BOUND: usize,
> BoundedNumber<T, UPPER_BOUND>
{
    pub fn wrapping_masked(value: T) -> Self {
        let _: () = AssertPowerOfTwo::<UPPER_BOUND>::OK;
        Self(value & ((T::ONE << (UPPER_BOUND.ilog2() as usize)) - T::ONE))
    }

    pub fn into_relaxed<const NEW_UPPER_BOUND: usize>(self) -> BoundedNumber<T, NEW_UPPER_BOUND> {
        let _: () = AssertLessEq::<UPPER_BOUND, NEW_UPPER_BOUND>::OK;
        BoundedNumber(self.0)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

pub type BoundedU8<const UPPER_BOUND: usize> = BoundedNumber<u8, UPPER_BOUND>;
pub type BoundedU16<const UPPER_BOUND: usize> = BoundedNumber<u16, UPPER_BOUND>;
pub type BoundedU32<const UPPER_BOUND: usize> = BoundedNumber<u32, UPPER_BOUND>;
pub type BoundedU64<const UPPER_BOUND: usize> = BoundedNumber<u64, UPPER_BOUND>;
pub type BoundedU128<const UPPER_BOUND: usize> = BoundedNumber<u128, UPPER_BOUND>;
pub type BoundedUsize<const UPPER_BOUND: usize> = BoundedNumber<usize, UPPER_BOUND>;

#[cfg(test)]
mod tests {
    use crate::bounded::BoundedNumber;

    #[test]
    fn test_masked() {
        let value: BoundedNumber<usize, 256> = BoundedNumber::wrapping_masked(321903);
        assert_eq!(value.0, 321903 % 256);
    }

    #[test]
    fn test_relaxation() {
        let value: BoundedNumber<usize, 65536> = BoundedNumber::wrapping_masked(23713);
        let value: BoundedNumber<usize, 100000> = value.into_relaxed();
        assert_eq!(value.0, 23713);
    }
}
