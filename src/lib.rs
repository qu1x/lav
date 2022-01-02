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
//!   * [`ApproxEq`] trait complementing [`PartialEq`].
//!   * Non-reflexive [`WrapFrom`] and [`WrapInto`] traits complementing [`From`] and [`Into`]
//!     without conflicting implementations.
//!   * [`Assert`] structure asserting constant generic expression when bound by trait [`True`].
//!   * [`no_std`] without loss of functionality by enabling the [`libm`] feature.
//!
//! This [`example`] uses SIMD generically over floating-point types while hiding it from the user.
//!
//! [Portable SIMD]: `core::simd`
//! [`Simd<f32, LANES>`]: `core::simd::Simd`
//! [`Simd<f64, LANES>`]: `core::simd::Simd`
//! [`Real::Simd<LANES>`]: `Real::Simd`
//! [`libm`]: https://docs.rs/libm
//! [`no_std`]: https://docs.rust-embedded.org/book/intro/no-std.html
//! [ULP]: https://en.wikipedia.org/wiki/Unit_in_the_last_place

#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![allow(clippy::type_repetition_in_bounds)]
#![allow(clippy::tabs_in_doc_comments)]
#![feature(generic_associated_types)]
#![feature(const_trait_impl)]
#![feature(const_fn_trait_bound)]
#![feature(portable_simd)]
#![feature(total_cmp)]
#![feature(convert_float_to_int)]
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
pub trait Select<M> {
	/// Selects lanes from two vectors by mask vector.
	///
	/// For each lane in the mask, choose the corresponding lane from `true_values` if that lane
	/// mask is true, and `false_values` if that lane mask is false.
	#[must_use]
	fn select(mask: M, true_values: Self, false_values: Self) -> Self;
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

/// [`From`] without reflexive `impl<T> `[`From`]`<T> for T`.
///
/// [`WrapFrom`]`<T> for U` implies (auto-implements) [`WrapInto`]`<U> for T`.
pub trait WrapFrom<T> {
	/// Performs the conversation.
	#[must_use]
	fn wrap_from(from: T) -> Self;
}

/// [`Into`] without reflexive `impl<U> `[`Into`]`<U> for U`.
///
/// Implied (auto-implemented) by [`WrapFrom`]`<T> for U`.
pub trait WrapInto<U> {
	/// Performs the conversation.
	#[must_use]
	fn wrap_into(self) -> U;
}

impl<T, U> const WrapInto<U> for T
where
	U: ~const WrapFrom<T>,
{
	#[inline]
	fn wrap_into(self) -> U {
		WrapFrom::wrap_from(self)
	}
}

/// Asserts constant generic expression `E` when bound by [`True`].
pub struct Assert<const E: bool> {}

/// Implemented for [`Assert`] with true expression.
pub trait True {}

impl True for Assert<true> {}
