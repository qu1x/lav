// Copyright © 2021 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::SimdBits;
use core::simd::{LaneCount, SimdElement, SupportedLaneCount};
use core::{
	fmt::{Debug, Display},
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
	ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
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
	Self: Debug + Display,
	Self: Add<Output = Self> + AddAssign,
	Self: Sub<Output = Self> + SubAssign,
	Self: Mul<Output = Self> + MulAssign,
	Self: Div<Output = Self> + DivAssign,
	Self: Rem<Output = Self> + RemAssign,
	Self: BitAnd<Output = Self> + BitAndAssign,
	Self: BitOr<Output = Self> + BitOrAssign,
	Self: BitXor<Output = Self> + BitXorAssign,
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

	/// Lanewise saturating add.
	fn saturating_add(self, other: Self) -> Self;
	/// Lanewise saturating subtract.
	fn saturating_sub(self, other: Self) -> Self;

	/// Lanewise absolute subtract.
	///
	/// Equals `self.saturating_sub(other) | other.saturating_sub(self)`.
	fn abs_sub(self, other: Self) -> Self {
		self.saturating_sub(other) | other.saturating_sub(self)
	}

	/// Constructs a SIMD vector by setting all lanes to the given value.
	fn splat<const LANES: usize>(self) -> Self::Simd<LANES>
	where
		LaneCount<LANES>: SupportedLaneCount,
	{
		Self::Simd::splat(self)
	}
}
