# Version 0.8.1 (2024-04-30)

  * Fix misnamed `Bits::{slice_as_simd,slice_as_simd_mut}`.
  * Provide instead of require `{Real,Bits}::{as_simd,as_simd_mut}`.

# Version 0.8.0 (2024-04-30)

  * Add `{Real,SimdReal,Bits,SimdBits}::{as_simd,as_simd_mut}`.
  * Update KaTeX.

# Version 0.7.8 (2023-12-10)

  * Fix KaTeX.

# Version 0.7.7 (2023-12-05)

  * Synchronize with nightly Rust.
  * Update KaTeX.

# Version 0.7.6 (2023-08-06)

  * Fix build by removing broken `const`-related features.
  * Adhere to pedantic lints using `Self` where possible.
  * Update KaTeX.

# Version 0.7.5 (2023-05-14)

  * Fix build by avoiding broken `const_option_ext` feature.

# Version 0.7.4 (2023-05-13)

  * Fix build by removing `Octal`/`LowerExp + UpperExp + Display` bound.
  * Fix build by using explicit `FromStr<Err = ParseFloatError>` bound.
  * Update KaTeX.

# Version 0.7.3 (2023-02-15)

  * Improve `FromStr<Err>` bound.
  * Add `R::NATIVE_LANE_COUNT` featured by crate `target-features`.

# Version 0.7.2 (2023-02-15)

  * Add `Send + Sync` and `FromStr<Err: Debug>` bounds.
  * Update KaTeX.

# Version 0.7.1 (2022-09-23)

  * Synchronize with nightly Rust (by marking traits with `const_trait`).

# Version 0.7.0 (2022-08-05)

  * Extend CI workflow.
  * Update KaTeX.
  * Rename prefix `lanes_` to `simd_`.
  * Synchronize with nightly Rust.

# Version 0.6.0 (2022-03-14)

  * Update KaTeX.
  * Rename prefix `horizontal_` to `reduce_`.
  * Synchronize with nightly Rust.

# Version 0.5.1 (2022-03-04)

  * Fix example.

# Version 0.5.0 (2022-03-04)

  * Add [`std::simd::StdFloat`] methods.
  * Add SIMD trait/type conversations.
  * Add [`FromUnchecked`] and [`IntoUnchecked`] traits.
  * Rename [`WrapFrom`] and [`WrapInto`] traits to [`PeelFrom`] and [`PeelInto`] traits.
  * Add [`WrapFromUnchecked`] and [`WrapIntoUnchecked`] traits.

# Version 0.4.0 (2022-01-03)

  * Add [`ApproxEq`] trait.
  * Add [`WrapFrom`] and [`WrapInto`] traits.
  * Add [`Assert`] structure and [`True`] trait.
  * Add [`Select`] trait.
  * Adhere to pedantic lints.

# Version 0.3.0 (2021-12-09)

  * Add trait bounds and their reference versions.
  * Remove SIMD trait bounds with scalar RHS.

# Version 0.2.1 (2021-12-05)

  * Use `must_use` and `inline` attributes.
  * Fix docs.

# Version 0.2.0 (2021-11-21)

  * Add [`SimdReal::mask_flag()`] and [`SimdMask::flag()`].
  * Swap arguments of [`SimdReal::insert()`].
  * Fix links and shorten description.

# Version 0.1.2 (2021-11-20)

  * Rephrase description.
  * Fix [`SimdReal::lanes_approx_ne()`].
  * Fix links and badge.

# Version 0.1.1 (2021-11-19)

  * Include KaTeX.

# Version 0.1.0 (2021-11-19)

  * Initial release.

[`ApproxEq`]:
https://docs.rs/lav/latest/lav/trait.ApproxEq.html
[`FromUnchecked`]:
https://docs.rs/lav/0.7.5/lav/trait.FromUnchecked.html
[`IntoUnchecked`]:
https://docs.rs/lav/0.7.5/lav/trait.IntoUnchecked.html
[`PeelFrom`]:
https://docs.rs/lav/0.7.5/lav/trait.PeelFrom.html
[`PeelInto`]:
https://docs.rs/lav/0.7.5/lav/trait.PeelInto.html
[`WrapFromUnchecked`]:
https://docs.rs/lav/0.7.5/lav/trait.WrapFromUnchecked.html
[`WrapIntoUnchecked`]:
https://docs.rs/lav/0.7.5/lav/trait.WrapIntoUnchecked.html
[`WrapFrom`]:
https://docs.rs/lav/0.4.0/lav/trait.WrapFrom.html
[`WrapInto`]:
https://docs.rs/lav/0.4.0/lav/trait.WrapInto.html
[`Assert`]:
https://docs.rs/lav/0.7.5/lav/struct.Assert.html
[`True`]:
https://docs.rs/lav/0.7.5/lav/trait.True.html
[`Select`]:
https://docs.rs/lav/latest/lav/trait.Select.html
[`SimdMask::flag()`]:
https://docs.rs/lav/latest/lav/trait.SimdMask.html#method.flag
[`SimdReal::mask_flag()`]:
https://docs.rs/lav/latest/lav/trait.SimdReal.html#method.mask_flag
[`SimdReal::lanes_approx_ne()`]:
https://docs.rs/lav/latest/lav/trait.SimdReal.html#method.lanes_approx_ne
[`SimdReal::insert()`]:
https://docs.rs/lav/latest/lav/trait.SimdReal.html#method.insert
[`std::simd::StdFloat`]:
https://doc.rust-lang.org/nightly/std/simd/trait.StdFloat.html
