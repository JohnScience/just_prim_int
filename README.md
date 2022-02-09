# Primitive integer marker traits

This crate provides [marker traits](https://blog.rust-lang.org/2015/05/11/traits.html#:~:text=Markers.,both%20generics%20and%20trait%20objects.) for [primitive integers](https://doc.rust-lang.org/reference/types/numeric.html#integer-types):

* [PrimInt](https://docs.rs/just_prim_int/latest/just_prim_int/trait.PrimInt.html);
* [PrimSignedInt](https://docs.rs/just_prim_int/latest/just_prim_int/trait.PrimSignedInt.html);
* [PrimUnsignedInt](https://docs.rs/just_prim_int/latest/just_prim_int/trait.PrimUnsignedInt.html).

# Numeric types

## Integer types

The unsigned integer types consist of:

Type   | Minimum | Maximum
-------|---------|-------------------
`u8`   | 0       | 2<sup>8</sup>-1
`u16`  | 0       | 2<sup>16</sup>-1
`u32`  | 0       | 2<sup>32</sup>-1
`u64`  | 0       | 2<sup>64</sup>-1
`u128` | 0       | 2<sup>128</sup>-1

The signed two's complement integer types consist of:

Type   | Minimum            | Maximum
-------|--------------------|-------------------
`i8`   | -(2<sup>7</sup>)   | 2<sup>7</sup>-1
`i16`  | -(2<sup>15</sup>)  | 2<sup>15</sup>-1
`i32`  | -(2<sup>31</sup>)  | 2<sup>31</sup>-1
`i64`  | -(2<sup>63</sup>)  | 2<sup>63</sup>-1
`i128` | -(2<sup>127</sup>) | 2<sup>127</sup>-1

## Machine-dependent integer types

The `usize` type is an unsigned integer type with the same number of bits as the
platform's pointer type. It can represent every memory address in the process.

The `isize` type is a signed integer type with the same number of bits as the
platform's pointer type. The theoretical upper bound on object and array size
is the maximum `isize` value. This ensures that `isize` can be used to calculate
differences between pointers into an object or array and can address every byte
within an object along with one byte past the end.

`usize` and `isize` are at least 16-bits wide.

# Example

## Cargo.toml

```toml
## ...

[dependencies]
just_prim_int = { version = "0.1.0" }

## ...

[features]

## https://doc.rust-lang.org/beta/unstable-book/language-features/marker-trait-attr.html
marker_trait_attr = ["just_prim_int/marker_trait_attr"]
```

## src/main.rs

```rust
// Uncomment if you want to conditionally use the feature. Remove otherwise.
// #![cfg_attr(feature = "marker_trait_attr", feature(marker_trait_attr))]

use just_prim_int::PrimInt;

trait MyExtensionTraitForPrimInts: PrimInt {
    // ...
}

fn main() {}
```

## Building, running, and testing

* `cargo build <other options> --features marker_trait_attr` if you want the feature and `cargo build <other options>` otherwise;
* `cargo run<other options> --features marker_trait_attr ` if you want the feature and `cargo build <other options>` otherwise;
* `cargo test <other options> --features marker_trait_attr` if you want the feature and `cargo test <other options>` otherwise.

# Features

With [`marker_trait_attr`](https://doc.rust-lang.org/beta/unstable-book/language-features/marker-trait-attr.html) [Nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) [feature](https://doc.rust-lang.org/cargo/reference/features.html), each of the provided traits also has `#[marker]` attribute which allows more optimal implementation of `PrimInt` via two "overlapping" generic `impl` blocks, one for `T: PrimSignedInt` and the other for `T: PrimUnsignedInt`. Without `#[marker]` attribute, they would be conflicting.

# Similar crates by the author:

* [`epui`](https://crates.io/crates/epui) - Equisized (primitive) unsigned ints for primitive ints: u8 for u8, u16 for i16, etc
* [`epsi`](https://crates.io/crates/epsi) - Equisized (primitive) signed ints for primitive ints
* [`primitive_promotion`](https://crates.io/crates/primitive_promotion) - Primitive promotions for primitive numeric types: u16 for u8, i32 for i16, f64 for f32, etc
* [`is_signed_trait`](https://crates.io/crates/is_signed_trait) - Trait for `IS_SIGNED` associated constant
* [`max_len_base_10_as_usize`](https://crates.io/crates/max_len_base_10_as_usize) - Trait offering constant maximum lengths of primitive integers as usize
* [`min_max_traits`](https://crates.io/crates/min_max_traits) - Traits for `MIN` and `MAX` associated constants
* as well as others that can be found on [crates.io](https://crates.io/users/JohnScience)

# Alternatives
* [`num_traits::int::PrimInt`](https://docs.rs/num-traits/latest/num_traits/int/trait.PrimInt.html) - At the moment of writing, [`num_traits`](https://crates.io/crates/num-traits) unconditionally provides non-const trait implementations with some useful functions.

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>