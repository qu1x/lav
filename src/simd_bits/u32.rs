// Copyright © 2021 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::SimdBits;
use core::simd::{LaneCount, Mask, Simd, SupportedLaneCount};

impl<const LANES: usize> SimdBits<u32, LANES> for Simd<u32, LANES>
where
	LaneCount<LANES>: SupportedLaneCount,
{
	type Mask = Mask<i32, LANES>;

	fn splat(value: u32) -> Self {
		Self::splat(value)
	}

	fn lanes_eq(self, other: Self) -> Self::Mask {
		self.lanes_eq(other)
	}
	fn lanes_ne(self, other: Self) -> Self::Mask {
		self.lanes_ne(other)
	}
	fn lanes_lt(self, other: Self) -> Self::Mask {
		self.lanes_lt(other)
	}
	fn lanes_gt(self, other: Self) -> Self::Mask {
		self.lanes_gt(other)
	}
	fn lanes_le(self, other: Self) -> Self::Mask {
		self.lanes_le(other)
	}
	fn lanes_ge(self, other: Self) -> Self::Mask {
		self.lanes_ge(other)
	}

	fn saturating_add(self, other: Self) -> Self {
		self.saturating_add(other)
	}
	fn saturating_sub(self, other: Self) -> Self {
		self.saturating_sub(other)
	}
}
