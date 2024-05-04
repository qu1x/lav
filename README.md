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

This [`example`] uses SIMD generically over floating-point types while hiding it from the user.

# Features

  * SIMD lane trait [`Real`] abstracting over [`f32`] and [`f64`].
  * SIMD vector trait [`SimdReal<Real, N>`] abstracting over `Simd<f32, N>` and `Simd<f64, N>`.
  * Generic associated type (GAT) [`Real::Simd<N>`] as part of SIMD lane trait [`Real`]
    implementing SIMD vector trait [`SimdReal<Self, N>`] for itself as lane type where the
    GAT is generic over the number of SIMD vector lanes `N`.
  * [AoS/SoA/AoSoA] via [`Real::as_simd`]/[`Real::as_simd_mut`] abstracting over
    [`as_simd`]/[`as_simd_mut`] of [`f32`] and [`f64`] slices.
  * Lanewise approximate equality test wrt to epsilon and [ULP] SIMD vectors.
  * [`ApproxEq`] trait complementing [`PartialEq`].

# Optional Features

Following features are disabled by default unless their feature gate is enabled:

  * [`target-features`]: Provides native number of SIMD vector lanes
    `Real::NATIVE_LANE_COUNT` for the current build target.
  * [`libm`]: Enables [`no_std`] without loss of functionality.

[Portable SIMD]: https://doc.rust-lang.org/nightly/core/simd/index.html
[`Real`]: https://docs.rs/lav/latest/lav/trait.Real.html
[`f32`]: https://doc.rust-lang.org/nightly/core/primitive.f32.html
[`f64`]: https://doc.rust-lang.org/nightly/core/primitive.f64.html
[`SimdReal<Real, N>`]: https://docs.rs/lav/latest/lav/trait.SimdReal.html
[`SimdReal<Self, N>`]: https://docs.rs/lav/latest/lav/trait.SimdReal.html
[`Real::Simd<N>`]: https://docs.rs/lav/latest/lav/trait.Real.html#associatedtype.Simd
[`Real::as_simd`]: https://docs.rs/lav/latest/lav/trait.Real.html#tymethod.as_simd
[`Real::as_simd_mut`]: https://docs.rs/lav/latest/lav/trait.Real.html#tymethod.as_simd_mut
[`as_simd`]: https://doc.rust-lang.org/nightly/core/primitive.slice.html#method.as_simd
[`as_simd_mut`]: https://doc.rust-lang.org/nightly/core/primitive.slice.html#method.as_simd_mut
[AoS/SoA/AoSoA]: https://en.wikipedia.org/wiki/AoS_and_SoA
[ULP]: https://en.wikipedia.org/wiki/Unit_in_the_last_place
[`ApproxEq`]: https://docs.rs/lav/latest/lav/trait.ApproxEq.html
[`PartialEq`]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html
[`target-features`]: https://docs.rs/target-features
[`libm`]: https://docs.rs/libm
[`no_std`]: https://docs.rust-embedded.org/book/intro/no-std.html
[`example`]: https://docs.rs/lav/latest/lav/example/index.html
[release history]: RELEASES.md

# Documentation Builds

Build and open documentation of this crate and its dependencies using KaTeX.

```sh
env RUSTDOCFLAGS="--html-in-header $PWD/katex.html" cargo doc --features target-features --open
```

Note that navigating the documentation requires web access as KaTeX is embedded via remote CDN.

## License

Copyright © 2021-2024 Rouven Spreckels <rs@qu1x.dev>

Licensed under the terms of the [`MPL-2.0`](LICENSES/MPL-2.0).

The MPL allows the integration of MPL-licensed code into proprietary codebases, as long as the
MPL-licensed components remain accessible under the terms of the MPL.

## Contribution

Unless you explicitly state otherwise, any Contribution intentionally submitted for inclusion in the
Covered Software by You shall be licensed as above, without any additional terms or conditions.
