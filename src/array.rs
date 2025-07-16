use std::{
    marker::PhantomData,
    ops::{Index, IndexMut, RangeTo},
    slice::{from_raw_parts, from_raw_parts_mut},
};

use crate::{
    bounded::BoundedUsize,
    const_assertions::{AssertLessEq, AssertLessEqPlusOne},
};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct BitBoundArray<T, const N: usize>(pub [T; N]);

impl<T: Default + Copy, const N: usize> Default for BitBoundArray<T, N> {
    fn default() -> Self {
        Self([T::default(); N])
    }
}

impl<T, const N: usize> BitBoundArray<T, N> {
    pub const fn from_array(array: [T; N]) -> Self {
        Self(array)
    }

    #[inline(always)]
    pub fn get_inner(&self) -> &[T; N] {
        &self.0
    }

    #[inline(always)]
    pub fn get_inner_mut(&mut self) -> &mut [T; N] {
        &mut self.0
    }

    pub fn len(&self) -> usize {
        N
    }
}

impl<T, const N: usize, const I: usize> Index<BoundedUsize<I>> for BitBoundArray<T, N> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: BoundedUsize<I>) -> &Self::Output {
        // SAFETY: Const assert that the index limit is leq than the current bound
        let _: () = AssertLessEq::<I, N>::OK;
        unsafe { self.0.get_unchecked(index.into_inner()) }
    }
}

impl<T, const N: usize, const I: usize> IndexMut<BoundedUsize<I>> for BitBoundArray<T, N> {
    #[inline(always)]
    fn index_mut(&mut self, index: BoundedUsize<I>) -> &mut Self::Output {
        // SAFETY: Const assert that the index limit is leq than the current bound
        let _: () = AssertLessEq::<I, N>::OK;
        unsafe { self.0.get_unchecked_mut(index.into_inner()) }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct BitBoundSlice<T, const N: usize>(PhantomData<T>, [()]);

impl<T, const N: usize, const I: usize> Index<BoundedUsize<I>> for BitBoundSlice<T, N> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: BoundedUsize<I>) -> &Self::Output {
        // SAFETY: Const assert that the index limit is leq than the current bound
        let _: () = AssertLessEq::<I, N>::OK;
        println!("Access: {}", index.into_inner());
        unsafe { &*(self.1.as_ptr() as *const T).add(index.into_inner()) }
    }
}

impl<T, const N: usize, const I: usize> IndexMut<BoundedUsize<I>> for BitBoundSlice<T, N> {
    #[inline(always)]
    fn index_mut(&mut self, index: BoundedUsize<I>) -> &mut Self::Output {
        // SAFETY: Const assert that the index limit is leq than the current bound
        let _: () = AssertLessEq::<I, N>::OK;
        unsafe { &mut *(self.1.as_mut_ptr() as *mut T).add(index.into_inner()) }
    }
}

impl<T, const N: usize> BitBoundSlice<T, N> {
    #[inline(always)]
    pub fn inner_slice(&self) -> &[T] {
        unsafe { from_raw_parts(self.1.as_ptr() as *const T, self.1.len()) }
    }

    #[inline(always)]
    pub fn inner_slice_mut(&mut self) -> &mut [T] {
        unsafe { from_raw_parts_mut(self.1.as_mut_ptr() as *mut T, self.1.len()) }
    }
}

impl<T, const N: usize> AsRef<BitBoundSlice<T, N>> for BitBoundArray<T, N> {
    #[inline(always)]
    fn as_ref(&self) -> &BitBoundSlice<T, N> {
        unsafe { std::mem::transmute(&self.0[..]) }
    }
}

impl<T, const N: usize, const I: usize> Index<RangeTo<BoundedUsize<I>>> for BitBoundArray<T, N> {
    type Output = BitBoundSlice<T, N>;

    #[inline(always)]
    fn index(&self, index: RangeTo<BoundedUsize<I>>) -> &Self::Output {
        // SAFETY: Const assert that the index limit is leq than the current bound
        // the resulting slice can be technically accessed out of bounds but always within the bounds of the original array
        let _: () = AssertLessEqPlusOne::<I, N>::OK;
        unsafe {
            let slice = self.0.get_unchecked(0..index.end.into_inner());
            std::mem::transmute(from_raw_parts(slice.as_ptr() as *const (), slice.len()))
        }
    }
}

impl<T, const N: usize, const I: usize> IndexMut<RangeTo<BoundedUsize<I>>> for BitBoundArray<T, N> {
    #[inline(always)]
    fn index_mut(&mut self, index: RangeTo<BoundedUsize<I>>) -> &mut Self::Output {
        // SAFETY: Const assert that the index limit is leq than the current bound
        // the resulting slice can be technically accessed out of bounds but always within the bounds of the original array
        let _: () = AssertLessEqPlusOne::<I, N>::OK;
        unsafe {
            let slice_mut = self.0.get_unchecked_mut(0..index.end.into_inner());
            std::mem::transmute(from_raw_parts_mut(
                slice_mut.as_mut_ptr() as *mut (),
                slice_mut.len(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{array::BitBoundArray, bounded::BoundedUsize};

    #[test]
    fn test_bitbound_array_index() {
        let array = BitBoundArray::from_array([1, 2, 3, 4]);
        let index = BoundedUsize::<2>::wrapping_masked(3);
        array[index];
    }

    #[test]
    fn test_bitbound_array_rangeindex() {
        let array = BitBoundArray::from_array([1, 2, 3, 4]);
        let index = unsafe { BoundedUsize::<5>::new_unchecked(4) };
        let slice = &array[..index];
        let slice_index = BoundedUsize::<4>::wrapping_masked(3);
        assert_eq!(array.0.as_ptr(), slice.inner_slice().as_ptr());
        slice[slice_index];
    }
}
