# lav

Lane-Associated Vector (LAV): [Portable SIMD] vector trait as GAT of SIMD lane trait.

[![Build][]](https://github.com/qu1x/lav/actions/workflows/build.yml)
[![Documentation][]](https://docs.rs/lav)
[![Downloads][]](https://crates.io/crates/lav)
[![Version][]](https://crates.io/crates/lav)
[![Rust][]](https://www.rust-lang.org)
[![License][]](https://mozilla.org/MPL)

[Build]: https://github.com/qu1x/lav/actions/workflows/build.yml/badge.svg
[Documentation]: https://docs.rs/lav/badge.svg
[Downloads]: https://img.shields.io/crates/d/lav.svg
[Version]: https://img.shields.io/crates/v/lav.svg
[Rust]: https://img.shields.io/badge/rust-nightly-orange.svg
[License]: https://img.shields.io/crates/l/lav

**NOTE**: This crate requires nightly Rust.

# Features

  * SIMD lane trait [`Real`] abstracting over [`f32`] and [`f64`].
  * SIMD vector trait [`SimdReal<Real, LANES>`] abstracting over [`Simd<f32, LANES>`] and
    [`Simd<f64, LANES>`].
  * Generic associated type (GAT) [`Real::Simd<LANES>`] as part of SIMD lane trait [`Real`]
    implementing SIMD vector trait [`SimdReal<Self, LANES>`] for itself as lane type where the
    GAT is generic over the number of SIMD vector `LANES`.
  * Lanewise approximate equality test wrt to epsilon and [ULP] SIMD vectors.
  * [`ApproxEq`] trait complementing [`PartialEq`].
  * Safe [`FromUnchecked`] and [`IntoUnchecked`] complementing [`From`] and [`Into`] where the
    behavior may be [unspecified] but will not result in undefined behavior if the caller breaks
    any logical constraint.
  * Non-reflexive [`PeelFrom`] and [`PeelInto`] traits complementing [`From`] and [`Into`]
    without conflicting implementations.
  * Safe [`WrapFromUnchecked`] and [`WrapIntoUnchecked`] complementing [`PeelFrom`] and
    [`PeelInto`] where the behavior may be [unspecified] but will not result in undefined
    behavior if the caller breaks any logical constraint.
  * [`Assert`] structure asserting constant generic expression when bound by trait [`True`].
  * [`no_std`] without loss of functionality by enabling the [`libm`] feature.

This [`example`] uses SIMD generically over floating-point types while hiding it from the user.

See the [release history] to keep track of the development.

[Portable SIMD]: https://doc.rust-lang.org/nightly/core/simd/index.html
[`f32`]: https://doc.rust-lang.org/nightly/core/primitive.f32.html
[`f64`]: https://doc.rust-lang.org/nightly/core/primitive.f64.html
[`Real`]: https://docs.rs/lav/latest/lav/trait.Real.html
[`SimdReal<Real, LANES>`]: https://docs.rs/lav/latest/lav/trait.SimdReal.html
[`SimdReal<Self, LANES>`]: https://docs.rs/lav/latest/lav/trait.SimdReal.html
[`Simd<f32, LANES>`]: https://doc.rust-lang.org/nightly/core/simd/struct.Simd.html#impl-10
[`Simd<f64, LANES>`]: https://doc.rust-lang.org/nightly/core/simd/struct.Simd.html#impl-11
[`Real::Simd<LANES>`]: https://docs.rs/lav/latest/lav/trait.Real.html#associatedtype.Simd
[ULP]: https://en.wikipedia.org/wiki/Unit_in_the_last_place
[unspecified]: https://doc.rust-lang.org/reference/behavior-not-considered-unsafe.html
[`ApproxEq`]: https://docs.rs/lav/latest/lav/trait.AapproxEq.html
[`PartialEq`]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html
[`FromUnchecked`]: https://docs.rs/lav/latest/lav/trait.FromUnchecked.html
[`IntoUnchecked`]: https://docs.rs/lav/latest/lav/trait.IntoUnchecked.html
[`From`]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html
[`Into`]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html
[`PeelFrom`]: https://docs.rs/lav/latest/lav/trait.PeelFrom.html
[`PeelInto`]: https://docs.rs/lav/latest/lav/trait.PeelInto.html
[`WrapFromUnchecked`]: https://docs.rs/lav/latest/lav/trait.WrapFromUnchecked.html
[`WrapIntoUnchecked`]: https://docs.rs/lav/latest/lav/trait.WrapIntoUnchecked.html
[`Assert`]: https://docs.rs/lav/latest/lav/struct.Assert.html
[`True`]: https://docs.rs/lav/latest/lav/trait.True.html
[`no_std`]: https://docs.rust-embedded.org/book/intro/no-std.html
[`libm`]: https://docs.rs/libm
[`example`]: https://docs.rs/lav/latest/lav/example/index.html
[release history]: RELEASES.md

# Documentation Builds

```sh
# Build and open documentation inclusive dependencies.
cargo doc --open
# Rebuild this crate's documentation with KaTeX.
cargo tex
# Refresh opened documentation.
```

With `cargo tex` defined in [.cargo/config](.cargo/config). Note that navigating the documentation
requires web access as KaTeX is embedded via remote CDN.

## License

Copyright ?? 2021-2022 Rouven Spreckels <rs@qu1x.dev>

Licensed under the terms of the [`MPL-2.0`](LICENSES/MPL-2.0).

The MPL allows the integration of MPL-licensed code into proprietary codebases, as long as the
MPL-licensed components remain accessible under the terms of the MPL.

## Contribution

Unless you explicitly state otherwise, any Contribution intentionally submitted for inclusion in the
Covered Software by You shall be licensed as above, without any additional terms or conditions.
