#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(feature = "marker_trait_attr", feature(marker_trait_attr))]

pub use {prim_signed_int::PrimSignedInt, prim_unsigned_int::PrimUnsignedInt};

/// [Marker trait](https://blog.rust-lang.org/2015/05/11/traits.html#:~:text=Markers.,both%20generics%20and%20trait%20objects.)
/// for [primitive integers](https://doc.rust-lang.org/reference/types/numeric.html#integer-types)
#[cfg_attr(feature = "marker_trait_attr", marker)]
pub trait PrimInt {}

#[cfg(not(feature = "marker_trait_attr"))]
mod prim_int_impl {
    impl super::PrimInt for u8 {}
    impl super::PrimInt for u16 {}
    impl super::PrimInt for u32 {}
    impl super::PrimInt for u64 {}
    impl super::PrimInt for u128 {}
    impl super::PrimInt for usize {}

    impl super::PrimInt for i8 {}
    impl super::PrimInt for i16 {}
    impl super::PrimInt for i32 {}
    impl super::PrimInt for i64 {}
    impl super::PrimInt for i128 {}
    impl super::PrimInt for isize {}
}

#[cfg(feature = "marker_trait_attr")]
mod prim_int_impl {
    impl<T: super::PrimUnsignedInt> super::PrimInt for T {}
    impl<T: super::PrimSignedInt> super::PrimInt for T {}
}

mod prim_unsigned_int {
    /// [Marker trait](https://blog.rust-lang.org/2015/05/11/traits.html#:~:text=Markers.,both%20generics%20and%20trait%20objects.)
    /// for [primitive unsigned integers](https://doc.rust-lang.org/reference/types/numeric.html#integer-types)
    #[cfg_attr(feature = "marker_trait_attr", marker)]
    pub trait PrimUnsignedInt: super::PrimInt {}

    impl PrimUnsignedInt for u8 {}
    impl PrimUnsignedInt for u16 {}
    impl PrimUnsignedInt for u32 {}
    impl PrimUnsignedInt for u64 {}
    impl PrimUnsignedInt for u128 {}
    impl PrimUnsignedInt for usize {}
}

mod prim_signed_int {
    /// [Marker trait](https://blog.rust-lang.org/2015/05/11/traits.html#:~:text=Markers.,both%20generics%20and%20trait%20objects.)
    /// for [primitive signed integers](https://doc.rust-lang.org/reference/types/numeric.html#integer-types)
    #[cfg_attr(feature = "marker_trait_attr", marker)]
    pub trait PrimSignedInt: super::PrimInt {}

    impl PrimSignedInt for i8 {}
    impl PrimSignedInt for i16 {}
    impl PrimSignedInt for i32 {}
    impl PrimSignedInt for i64 {}
    impl PrimSignedInt for i128 {}
    impl PrimSignedInt for isize {}
}

pub struct AbstrPrimIntZST;

impl AbstrPrimIntZST {
    pub const MAX_SIZE: usize = 16;
    pub const MIN_SIZE: usize = 1;
}

#[cfg(test)]
mod tests {
    use crate::{ PrimInt, AbstrPrimIntZST };

    fn do_nothing_for_prim_int_ref<T: PrimInt>(_ref: &T) {}

    #[test]
    fn u8_is_prim_int() {
        do_nothing_for_prim_int_ref::<u8>(&5)
    }

    #[test]
    fn max_size_of_abstr_prim_int_is_that_of_u128() {
        assert_eq!(AbstrPrimIntZST::MAX_SIZE, core::mem::size_of::<u128>());
    }

    #[test]
    fn min_size_of_abstr_prim_int_is_that_of_u8() {
        assert_eq!(AbstrPrimIntZST::MIN_SIZE, core::mem::size_of::<u8>());
    }
}
