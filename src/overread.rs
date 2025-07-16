use std::ops::Deref;

pub struct OverreadVec<T, const OVERREAD: usize>(Vec<T>);

impl<T, const OVERREAD: usize> OverreadVec<T, OVERREAD> {
    #[inline(always)]
    pub fn new() -> Self {
        Self(Vec::with_capacity(OVERREAD))
    }

    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity + OVERREAD))
    }

    #[inline(always)]
    pub fn from_vec(vec: Vec<T>) -> Self {
        let mut vec = vec;
        vec.reserve(OVERREAD);
        Self(vec)
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    #[inline(always)]
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional + OVERREAD);
    }

    #[inline(always)]
    pub fn push(&mut self, value: T) {
        self.0.push(value);
        self.0.reserve(OVERREAD);
    }

    #[inline(always)]
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }

    #[inline(always)]
    pub fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter);
        self.0.reserve(OVERREAD);
    }

    #[inline(always)]
    pub fn extend_from_slice(&mut self, slice: &[T])
    where
        T: Clone,
    {
        self.0.extend_from_slice(slice);
        self.0.reserve(OVERREAD);
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    #[inline(always)]
    pub fn insert(&mut self, index: usize, element: T) {
        self.0.insert(index, element);
        self.0.reserve(OVERREAD);
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.0.as_mut_ptr()
    }
}

impl<T, const OVERREAD: usize> AsRef<Vec<T>> for OverreadVec<T, OVERREAD> {
    #[inline(always)]
    fn as_ref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T, const OVERREAD: usize> Deref for OverreadVec<T, OVERREAD> {
    type Target = Vec<T>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
