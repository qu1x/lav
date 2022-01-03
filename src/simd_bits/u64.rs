// Copyright © 2021-2022 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{Select, SimdBits};
use core::simd::{LaneCount, Mask, Simd, SupportedLaneCount};

impl<const LANES: usize> SimdBits<u64, LANES> for Simd<u64, LANES>
where
	LaneCount<LANES>: SupportedLaneCount,
{
	type Mask = Mask<i64, LANES>;

	#[inline]
	fn splat(value: u64) -> Self {
		Self::splat(value)
	}

	#[inline]
	fn lanes_eq(self, other: Self) -> Self::Mask {
		self.lanes_eq(other)
	}
	#[inline]
	fn lanes_ne(self, other: Self) -> Self::Mask {
		self.lanes_ne(other)
	}
	#[inline]
	fn lanes_lt(self, other: Self) -> Self::Mask {
		self.lanes_lt(other)
	}
	#[inline]
	fn lanes_gt(self, other: Self) -> Self::Mask {
		self.lanes_gt(other)
	}
	#[inline]
	fn lanes_le(self, other: Self) -> Self::Mask {
		self.lanes_le(other)
	}
	#[inline]
	fn lanes_ge(self, other: Self) -> Self::Mask {
		self.lanes_ge(other)
	}

	#[inline]
	fn saturating_add(self, other: Self) -> Self {
		self.saturating_add(other)
	}
	#[inline]
	fn saturating_sub(self, other: Self) -> Self {
		self.saturating_sub(other)
	}
}

impl<const LANES: usize> Select<Mask<i64, LANES>> for Simd<u64, LANES>
where
	LaneCount<LANES>: SupportedLaneCount,
{
	#[inline]
	fn select(mask: Mask<i64, LANES>, true_values: Self, false_values: Self) -> Self {
		mask.select(true_values, false_values)
	}
}
