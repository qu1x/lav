// Copyright © 2021 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{Bits, SimdMask};
use core::{
	fmt::Debug,
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
	ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
	ops::{Index, IndexMut},
	simd::Select,
};

mod u32;
mod u64;

/// Bits representation vector of [`SimdReal`] vector with associated [`SimdMask`] vector.
///
/// [`SimdReal`]: `super::SimdReal`
pub trait SimdBits<B: Bits, const LANES: usize>
where
	Self: Clone + Copy + Default,
	Self: PartialEq + Eq + PartialOrd + Ord,
	Self: Debug,
	Self: From<[B; LANES]> + AsRef<[B; LANES]> + AsMut<[B; LANES]>,
	Self: Index<usize, Output = B> + IndexMut<usize, Output = B>,
	Self: Select<Self::Mask>,
	Self: Add<Output = Self> + AddAssign + Add<B, Output = Self> + AddAssign<B>,
	Self: Sub<Output = Self> + SubAssign + Sub<B, Output = Self> + SubAssign<B>,
	Self: Mul<Output = Self> + MulAssign + Mul<B, Output = Self> + MulAssign<B>,
	Self: Div<Output = Self> + DivAssign + Div<B, Output = Self> + DivAssign<B>,
	Self: Rem<Output = Self> + RemAssign + Rem<B, Output = Self> + RemAssign<B>,
	Self: BitAnd<Output = Self> + BitAndAssign,
	Self: BitOr<Output = Self> + BitOrAssign,
	Self: BitXor<Output = Self> + BitXorAssign,
	Self: Not<Output = Self>,
{
	/// Associated mask vector.
	type Mask: SimdMask<LANES>;

	/// Number of lanes in this vector.
	const LANES: usize = LANES;

	/// Get the number of lanes in this vector.
	fn lanes(&self) -> usize {
		LANES
	}

	/// Constructs a SIMD vector by setting all lanes to the given value.
	fn splat(value: B) -> Self;

	/// Test if each lane is equal to the corresponding lane in `other`.
	fn lanes_eq(self, other: Self) -> Self::Mask;
	/// Test if each lane is not equal to the corresponding lane in `other`.
	fn lanes_ne(self, other: Self) -> Self::Mask;
	/// Test if each lane is less than the corresponding lane in `other`.
	fn lanes_lt(self, other: Self) -> Self::Mask;
	/// Test if each lane is greater than the corresponding lane in `other`.
	fn lanes_gt(self, other: Self) -> Self::Mask;
	/// Test if each lane is less than or equal to the corresponding lane in `other`.
	fn lanes_le(self, other: Self) -> Self::Mask;
	/// Test if each lane is greater than or equal to the corresponding lane in `other`.
	fn lanes_ge(self, other: Self) -> Self::Mask;

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
}
