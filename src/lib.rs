// Copyright © 2021-2024 Rouven Spreckels <rs@qu1x.dev>
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
//!   * SIMD vector trait [`SimdReal<Real, N>`] abstracting over [`Simd<f32, N>`] and
//!     [`Simd<f64, N>`].
//!   * Generic associated type (GAT) [`Real::Simd<N>`] as part of SIMD lane trait [`Real`]
//!     implementing SIMD vector trait [`SimdReal<Self, N>`] for itself as lane type where the
//!     GAT is generic over the number of SIMD vector lanes `N`.
//!   * Supports [AoS/SoA/AoSoA] via [`Real::as_simd`]/[`Real::as_simd_mut`] abstracting over
//!     [`as_simd`]/[`as_simd_mut`] of [`f32`] and [`f64`] slices.
//!   * Lanewise approximate equality test wrt to epsilon and [ULP] SIMD vectors.
//!   * [`ApproxEq`] trait complementing [`PartialEq`].
//!   * [`no_std`] without loss of functionality by enabling the [`libm`] feature.
//!
//! This [`example`] uses SIMD generically over floating-point types while hiding it from the user.
//!
//! [Portable SIMD]: `core::simd`
//! [`Simd<f32, N>`]: `core::simd::Simd`
//! [`Simd<f64, N>`]: `core::simd::Simd`
//! [`Real::Simd<N>`]: `Real::Simd`
//! [`as_simd`]: `slice::as_simd`
//! [`as_simd_mut`]: `slice::as_simd_mut`
//! [`libm`]: https://docs.rs/libm
//! [`no_std`]: https://docs.rust-embedded.org/book/intro/no-std.html
//! [AoS/SoA/AoSoA]: https://en.wikipedia.org/wiki/AoS_and_SoA
//! [ULP]: https://en.wikipedia.org/wiki/Unit_in_the_last_place

#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(clippy::type_repetition_in_bounds)]
#![allow(clippy::trait_duplication_in_bounds)]
#![allow(clippy::tabs_in_doc_comments)]
#![feature(decl_macro)]
#![feature(portable_simd)]
#![feature(convert_float_to_int)]
#![feature(doc_auto_cfg)]
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

/// Selects lanes from two vectors by mask vector.
pub trait Select<Mask> {
	/// Selects lanes from two vectors by mask vector.
	///
	/// For each lane in the mask, choose the corresponding lane from `true_values` if that lane
	/// mask is true, and `false_values` if that lane mask is false.
	#[must_use]
	fn select(mask: Mask, true_values: Self, false_values: Self) -> Self;
}

/// Tests for approximate equality.
pub trait ApproxEq<R: Real, Rhs = Self>
where
	Rhs: ?Sized,
{
	/// Tests for approximate equality wrt `epsilon` or `ulp`, "or" in the sense of `||`.
	#[must_use]
	fn approx_eq(&self, other: &Rhs, epsilon: R, ulp: R::Bits) -> bool;
	/// Tests for approximate inequality wrt `epsilon` and `ulp`, "and" in the sense of `&&`.
	#[must_use]
	#[inline]
	fn approx_ne(&self, other: &Rhs, epsilon: R, ulp: R::Bits) -> bool {
		!self.approx_eq(other, epsilon, ulp)
	}
}
