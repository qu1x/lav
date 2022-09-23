// Copyright © 2021-2022 Rouven Spreckels <rs@qu1x.dev>
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
//!   * Safe [`FromUnchecked`] and [`IntoUnchecked`] complementing [`From`] and [`Into`] where the
//!     behavior may be [unspecified] but will not result in undefined behavior if the caller breaks
//!     any logical constraint.
//!   * Non-reflexive [`PeelFrom`] and [`PeelInto`] traits complementing [`From`] and [`Into`]
//!     without conflicting implementations.
//!   * Safe [`WrapFromUnchecked`] and [`WrapIntoUnchecked`] complementing [`PeelFrom`] and
//!     [`PeelInto`] where the behavior may be [unspecified] but will not result in undefined
//!     behavior if the caller breaks any logical constraint.
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
//! [unspecified]: https://doc.rust-lang.org/reference/behavior-not-considered-unsafe.html

#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(clippy::type_repetition_in_bounds)]
#![allow(clippy::trait_duplication_in_bounds)]
#![allow(clippy::tabs_in_doc_comments)]
#![feature(const_trait_impl)]
#![feature(portable_simd)]
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

/// Compile-time unchecked but safe [`From`].
///
/// **Note:** This trait must not fail.
///
/// # Unchecked
///
/// If the caller breaks any logical constraint, the behavior may be [unspecified] but will not
/// result in undefined behavior.
///
/// [unspecified]: https://doc.rust-lang.org/reference/behavior-not-considered-unsafe.html
///
/// # Generic Implementations
///
///   * [`FromUnchecked`]`<T> for U` implies (auto-implements) [`IntoUnchecked`]`<U> for T`.
///   * [`FromUnchecked`]`<T>` implies (auto-implements) [`FromUnchecked`]`<T> for T` (reflexive).
#[const_trait]
pub trait FromUnchecked<T> {
	/// Performs the conversation.
	#[must_use]
	fn from_unchecked(from: T) -> Self;
}

impl<T> FromUnchecked<T> for T {
	#[inline]
	fn from_unchecked(from: T) -> T {
		from
	}
}

/// Compile-time unchecked but safe [`Into`].
///
/// **Note:** This trait must not fail.
///
/// # Unchecked
///
/// If the caller breaks any logical constraint, the behavior may be [unspecified] but will not
/// result in undefined behavior.
///
/// [unspecified]: https://doc.rust-lang.org/reference/behavior-not-considered-unsafe.html
///
/// # Generic Implementations
///
///   * Implied (auto-implemented) by [`FromUnchecked`]`<T> for U`.
#[const_trait]
pub trait IntoUnchecked<U> {
	/// Performs the conversation.
	#[must_use]
	fn into_unchecked(self) -> U;
}

impl<T, U> const IntoUnchecked<U> for T
where
	U: ~const FromUnchecked<T>,
{
	#[inline]
	fn into_unchecked(self) -> U {
		FromUnchecked::from_unchecked(self)
	}
}

/// [`From`] without reflexive `impl<T> `[`From`]`<T> for T`.
///
/// **Note:** This trait must not fail.
///
/// # Generic Implementations
///
///   * [`PeelFrom`]`<T> for U` implies (auto-implements) [`PeelInto`]`<U> for T`.
#[const_trait]
pub trait PeelFrom<T> {
	/// Performs the conversation.
	#[must_use]
	fn peel_from(from: T) -> Self;
}

/// [`Into`] without reflexive `impl<U> `[`Into`]`<U> for U`.
///
/// **Note:** This trait must not fail.
///
/// # Generic Implementations
///
///   * Implied (auto-implemented) by [`PeelFrom`]`<T> for U`.
#[const_trait]
pub trait PeelInto<U> {
	/// Performs the conversation.
	#[must_use]
	fn peel_into(self) -> U;
}

impl<T, U> const PeelInto<U> for T
where
	U: ~const PeelFrom<T>,
{
	#[inline]
	fn peel_into(self) -> U {
		PeelFrom::peel_from(self)
	}
}

/// Compile-time unchecked but safe inverse of [`PeelFrom`].
///
/// **Note:** This trait must not fail.
///
/// # Unchecked
///
/// If the caller breaks any logical constraint, the behavior may be [unspecified] but will not
/// result in undefined behavior.
///
/// [unspecified]: https://doc.rust-lang.org/reference/behavior-not-considered-unsafe.html
///
/// # Generic Implementations
///
///   * [`WrapFromUnchecked`]`<T> for U` implies (auto-implements) [`WrapIntoUnchecked`]`<U> for T`.
#[const_trait]
pub trait WrapFromUnchecked<T> {
	/// Performs the conversation.
	#[must_use]
	fn wrap_from_unchecked(from: T) -> Self;
}

/// Compile-time unchecked but safe inverse of [`PeelInto`].
///
/// **Note:** This trait must not fail.
///
/// # Unchecked
///
/// If the caller breaks any logical constraint, the behavior may be [unspecified] but will not
/// result in undefined behavior.
///
/// [unspecified]: https://doc.rust-lang.org/reference/behavior-not-considered-unsafe.html
///
/// # Generic Implementations
///
///   * Implied (auto-implemented) by [`WrapFromUnchecked`]`<T> for U`.
#[const_trait]
pub trait WrapIntoUnchecked<U> {
	/// Performs the conversation.
	#[must_use]
	fn wrap_into_unchecked(self) -> U;
}

impl<T, U> const WrapIntoUnchecked<U> for T
where
	U: ~const WrapFromUnchecked<T>,
{
	#[inline]
	fn wrap_into_unchecked(self) -> U {
		WrapFromUnchecked::wrap_from_unchecked(self)
	}
}

/// Asserts constant generic expression `E` when bound by [`True`].
pub struct Assert<const E: bool> {}

/// Implemented for [`Assert`] with true expression.
#[const_trait]
pub trait True {}

impl const True for Assert<true> {}
