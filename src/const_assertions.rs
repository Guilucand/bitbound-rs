pub struct AssertPowerOfTwo<const N: usize>;
pub struct AssertLessEq<const N: usize, const M: usize>;

pub struct AssertLessEqPlusOne<const N: usize, const M: usize>;

impl<const N: usize> AssertPowerOfTwo<N> {
    pub const OK: () = assert!(N.next_power_of_two() == N, "must be a power of 2");
}

impl<const N: usize, const M: usize> AssertLessEq<N, M> {
    pub const OK: () = assert!(N <= M, "N must be less or equal than M");
}

impl<const N: usize, const M: usize> AssertLessEqPlusOne<N, M> {
    pub const OK: () = assert!(N <= M + 1, "N must be less or equal than M + 1");
}
