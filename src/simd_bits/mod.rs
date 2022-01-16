// Copyright © 2021-2022 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{Bits, Select, SimdMask};
use core::{
	fmt::{Debug, Octal},
	hash::Hash,
	iter::{Product, Sum},
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
	ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
	ops::{Index, IndexMut},
	ops::{Shl, ShlAssign, Shr, ShrAssign},
	simd::{LaneCount, Simd, SupportedLaneCount},
};

mod u32;
mod u64;

/// Bits representation vector of [`SimdReal`] vector with associated [`SimdMask`] vector.
///
/// [`SimdReal`]: `super::SimdReal`
pub trait SimdBits<B: Bits, const LANES: usize>
where
	LaneCount<LANES>: SupportedLaneCount,
	Self: Clone + Copy + Default,
	Self: PartialEq + Eq + PartialOrd + Ord,
	Self: From<Simd<B, LANES>> + Into<Simd<B, LANES>>,
	Self: From<[B; LANES]> + Into<[B; LANES]>,
	Self: AsRef<[B; LANES]> + AsMut<[B; LANES]>,
	Self: Product<Self> + Sum<Self>,
	for<'a> Self: Product<&'a Self> + Sum<&'a Self>,
	Self: Hash,
	Self: Debug + Octal,
	Self: Index<usize, Output = B> + IndexMut<usize, Output = B>,
	Self: Select<Self::Mask>,
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
{
	/// Associated mask vector.
	type Mask: SimdMask<LANES>;

	/// Number of lanes in this vector.
	const LANES: usize = LANES;

	/// Get the number of lanes in this vector.
	#[must_use]
	#[inline]
	fn lanes(&self) -> usize {
		LANES
	}

	/// Constructs a SIMD vector by setting all lanes to the given value.
	#[must_use]
	fn splat(value: B) -> Self;

	/// Test if each lane is equal to the corresponding lane in `other`.
	#[must_use]
	fn lanes_eq(self, other: Self) -> Self::Mask;
	/// Test if each lane is not equal to the corresponding lane in `other`.
	#[must_use]
	fn lanes_ne(self, other: Self) -> Self::Mask;
	/// Test if each lane is less than the corresponding lane in `other`.
	#[must_use]
	fn lanes_lt(self, other: Self) -> Self::Mask;
	/// Test if each lane is greater than the corresponding lane in `other`.
	#[must_use]
	fn lanes_gt(self, other: Self) -> Self::Mask;
	/// Test if each lane is less than or equal to the corresponding lane in `other`.
	#[must_use]
	fn lanes_le(self, other: Self) -> Self::Mask;
	/// Test if each lane is greater than or equal to the corresponding lane in `other`.
	#[must_use]
	fn lanes_ge(self, other: Self) -> Self::Mask;

	/// Lanewise saturating add.
	#[must_use]
	fn saturating_add(self, other: Self) -> Self;
	/// Lanewise saturating subtract.
	#[must_use]
	fn saturating_sub(self, other: Self) -> Self;

	/// Lanewise absolute subtract.
	///
	/// Equals `self.saturating_sub(other) | other.saturating_sub(self)`.
	#[must_use]
	#[inline]
	fn abs_sub(self, other: Self) -> Self {
		self.saturating_sub(other) | other.saturating_sub(self)
	}
}
