// Copyright © 2021 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::SimdBits;
use core::{
	fmt::{Debug, Display, Octal},
	hash::Hash,
	iter::{Product, Sum},
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
	ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
	ops::{Shl, ShlAssign, Shr, ShrAssign},
	simd::{LaneCount, SimdElement, SupportedLaneCount},
};

mod u32;
mod u64;

/// Bits representation of [`Real`] number with associated [`SimdBits`] vector.
///
/// [`Real`]: `super::Real`
pub trait Bits
where
	Self: Clone + Copy + Default,
	Self: PartialEq + Eq + PartialOrd + Ord,
	Self: Product<Self> + Sum<Self>,
	for<'a> Self: Product<&'a Self> + Sum<&'a Self>,
	Self: Hash,
	Self: Debug + Octal + Display,
	Self: Add<Output = Self> + AddAssign,
	Self: Sub<Output = Self> + SubAssign,
	Self: Mul<Output = Self> + MulAssign,
	Self: Div<Output = Self> + DivAssign,
	Self: Rem<Output = Self> + RemAssign,
	Self: Shl<Output = Self> + ShlAssign,
	Self: Shr<Output = Self> + ShrAssign,
	Self: BitAnd<Output = Self> + BitAndAssign,
	Self: BitOr<Output = Self> + BitOrAssign,
	Self: BitXor<Output = Self> + BitXorAssign,
	for<'a> Self: Add<&'a Self, Output = Self> + AddAssign<&'a Self>,
	for<'a> Self: Sub<&'a Self, Output = Self> + SubAssign<&'a Self>,
	for<'a> Self: Mul<&'a Self, Output = Self> + MulAssign<&'a Self>,
	for<'a> Self: Div<&'a Self, Output = Self> + DivAssign<&'a Self>,
	for<'a> Self: Rem<&'a Self, Output = Self> + RemAssign<&'a Self>,
	for<'a> Self: Shl<&'a Self, Output = Self> + ShlAssign<&'a Self>,
	for<'a> Self: Shr<&'a Self, Output = Self> + ShrAssign<&'a Self>,
	for<'a> Self: BitAnd<&'a Self, Output = Self> + BitAndAssign<&'a Self>,
	for<'a> Self: BitOr<&'a Self, Output = Self> + BitOrAssign<&'a Self>,
	for<'a> Self: BitXor<&'a Self, Output = Self> + BitXorAssign<&'a Self>,
	Self: Not<Output = Self>,
	Self: SimdElement,
{
	/// Associated vector.
	type Simd<const LANES: usize>: SimdBits<Self, LANES>
	where
		LaneCount<LANES>: SupportedLaneCount;

	/// The smallest value that can be represented by this integer type.
	const MIN: Self;
	/// The largest value that can be represented by this integer type.
	const MAX: Self;

	/// $1$
	const ONE: Self;

	/// Saturating add.
	#[must_use]
	fn saturating_add(self, other: Self) -> Self;
	/// Saturating subtract.
	#[must_use]
	fn saturating_sub(self, other: Self) -> Self;

	/// Absolute subtract.
	///
	/// Equals `self.saturating_sub(other) | other.saturating_sub(self)`.
	#[must_use]
	#[inline]
	fn abs_sub(self, other: Self) -> Self {
		self.saturating_sub(other) | other.saturating_sub(self)
	}

	/// Constructs a SIMD vector by setting all lanes to the given value.
	#[must_use]
	#[inline]
	fn splat<const LANES: usize>(self) -> Self::Simd<LANES>
	where
		LaneCount<LANES>: SupportedLaneCount,
	{
		Self::Simd::splat(self)
	}
}
