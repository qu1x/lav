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
  * SIMD vector trait [`SimdReal<Real, N>`] abstracting over `Simd<f32, N>` and `Simd<f64, N>`.
  * Generic associated type (GAT) [`Real::Simd<N>`] as part of SIMD lane trait [`Real`]
    implementing SIMD vector trait [`SimdReal<Self, N>`] for itself as lane type where the
    GAT is generic over the number of SIMD vector lanes `N`.
  * Lanewise approximate equality test wrt to epsilon and [ULP] SIMD vectors.
  * [`ApproxEq`] trait complementing [`PartialEq`].
  * [`no_std`] without loss of functionality by enabling the [`libm`] feature.

This [`example`] uses SIMD generically over floating-point types while hiding it from the user.

See the [release history] to keep track of the development.

[Portable SIMD]: https://doc.rust-lang.org/nightly/core/simd/index.html
[`Real`]: https://docs.rs/lav/latest/lav/trait.Real.html
[`f32`]: https://doc.rust-lang.org/nightly/core/primitive.f32.html
[`f64`]: https://doc.rust-lang.org/nightly/core/primitive.f64.html
[`SimdReal<Real, N>`]: https://docs.rs/lav/latest/lav/trait.SimdReal.html
[`SimdReal<Self, N>`]: https://docs.rs/lav/latest/lav/trait.SimdReal.html
[`Real::Simd<N>`]: https://docs.rs/lav/latest/lav/trait.Real.html#associatedtype.Simd
[ULP]: https://en.wikipedia.org/wiki/Unit_in_the_last_place
[`ApproxEq`]: https://docs.rs/lav/latest/lav/trait.ApproxEq.html
[`PartialEq`]: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html
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

Copyright © 2021-2022 Rouven Spreckels <rs@qu1x.dev>

Licensed under the terms of the [`MPL-2.0`](LICENSES/MPL-2.0).

The MPL allows the integration of MPL-licensed code into proprietary codebases, as long as the
MPL-licensed components remain accessible under the terms of the MPL.

## Contribution

Unless you explicitly state otherwise, any Contribution intentionally submitted for inclusion in the
Covered Software by You shall be licensed as above, without any additional terms or conditions.
