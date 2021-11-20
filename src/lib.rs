// Copyright © 2021 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Lane-Associated Vector (LAV): [Portable SIMD] vector trait as GAT of SIMD lane trait.
//!
//! **NOTE**: This crate requires nightly Rust.
//!
//! # Features
//!
//!   * SIMD lane trait [`Real`] abstracting over [`f32`] and [`f64`].
//!   * SIMD vector trait [`SimdReal<Real, LANES>`] abstracting over [`Simd<f32, LANES>`] and
//!     [`Simd<f64, LANES>`].
//!   * Generic associated type (GAT) [`Real::Simd<LANES>`] as part of SIMD lane trait [`Real`]
//!     implementing SIMD vector trait [`SimdReal<Self, LANES>`] for itself as lane type where the
//!     GAT is generic over the number of SIMD vector `LANES`.
//!   * Lanewise approximate equality test wrt to epsilon and [ULP] SIMD vectors.
//!   * [`no_std`] without loss of functionality by enabling the [`libm`] feature.
//!
//! This [`example`] uses SIMD generically over floating-point types while hiding it from the user.
//! This is useful for crates that can be efficiently implemented without [AoS, SoA, or AoSoA] like
//! the projective geometric algebra crate [`pga`] (work in progress).
//!
//! [Portable SIMD]: `core::simd`
//! [`Simd<f32, LANES>`]: `core::simd::Simd`
//! [`Simd<f64, LANES>`]: `core::simd::Simd`
//! [`Real::Simd<LANES>`]: `Real::Simd`
//! [`pga`]: https:/docs.rs/pga
//! [`libm`]: https:/docs.rs/libm
//! [`no_std`]: https://docs.rust-embedded.org/book/intro/no-std.html
//! [ULP]: https://en.wikipedia.org/wiki/Unit_in_the_last_place
//! [AoS, SoA, or AoSoA]: https://en.wikipedia.org/wiki/AoS_and_SoA

#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![allow(clippy::tabs_in_doc_comments)]
#![feature(generic_associated_types)]
#![feature(portable_simd)]
#![feature(total_cmp)]
#![cfg_attr(feature = "libm", no_std)]

mod bits;
mod real;
mod simd_bits;
mod simd_mask;
mod simd_real;

pub use bits::*;
pub use real::*;
pub use simd_bits::*;
pub use simd_mask::*;
pub use simd_real::*;

pub mod example;
