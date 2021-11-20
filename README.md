# lav

Lane-Associated Vector (LAV): [Portable SIMD] vector trait as GAT of SIMD lane trait.

[![Build][]](https://github.com/qu1x/lav/actions/workflows/build.yml)
[![Downloads][]](https://crates.io/crates/lav)
[![Rust][]](https://www.rust-lang.org)
[![Version][]](https://crates.io/crates/lav)
[![Documentation][]](https://docs.rs/lav)
[![License][]](https://mozilla.org/MPL)

[Build]: https://github.com/qu1x/lav/actions/workflows/build.yml/badge.svg
[Downloads]: https://img.shields.io/crates/d/lav.svg
[Rust]: https://img.shields.io/badge/rust-nightly-orange.svg
[Version]: https://img.shields.io/crates/v/lav.svg
[Documentation]: https://docs.rs/lav/badge.svg
[License]: https://img.shields.io/crates/l/lav

**NOTE**: This crate requires nightly Rust.

# Features

  * SIMD lane trait `Real` abstracting over `f32` and `f64`.
  * SIMD vector trait `SimdReal<Real, LANES>` abstracting over `Simd<f32, LANES>` and
    `Simd<f64, LANES>`.
  * Generic associated type (GAT) `Real::Simd<LANES>` as part of SIMD lane trait `Real`
    implementing SIMD vector trait `SimdReal<Self, LANES>` for itself as lane type where the
    GAT is generic over the number of SIMD vector `LANES`.
  * Lanewise approximate equality test wrt to epsilon and [ULP] SIMD vectors.
  * [`no_std`] without loss of functionality by enabling the [`libm`] feature.

This [`example`] uses SIMD generically over floating-point types while hiding it from the user.
This is useful for crates that can be efficiently implemented without [AoS, SoA, or AoSoA] like
the projective geometric algebra crate [`pga`] (work in progress).

[Portable SIMD]: https://doc.rust-lang.org/nightly/core/simd/index.html
[`example`]: src/example/mod.rs
[`pga`]: https://docs.rs/pga
[`libm`]: https://docs.rs/libm
[`no_std`]: https://docs.rust-embedded.org/book/intro/no-std.html
[ULP]: https://en.wikipedia.org/wiki/Unit_in_the_last_place
[AoS, SoA, or AoSoA]: https://en.wikipedia.org/wiki/AoS_and_SoA

See the [release history] to keep track of the development.

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

Copyright © 2021 Rouven Spreckels <rs@qu1x.dev>

Licensed under the terms of the [`MPL-2.0`](LICENSES/MPL-2.0).

The MPL allows the integration of MPL-licensed code into proprietary codebases, as long as the
MPL-licensed components remain accessible under the terms of the MPL.

## Contribution

Unless you explicitly state otherwise, any Contribution intentionally submitted for inclusion in the
Covered Software by You shall be licensed as above, without any additional terms or conditions.
