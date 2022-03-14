// Copyright © 2021 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::Bits;
use core::simd::{LaneCount, Simd, SupportedLaneCount};

impl Bits for u64 {
	type Simd<const LANES: usize> = Simd<Self, LANES>
	where
		LaneCount<LANES>: SupportedLaneCount;

	const MIN: Self = u64::MIN;
	const MAX: Self = u64::MAX;

	const ONE: Self = 1;

	#[inline]
	fn saturating_add(self, other: Self) -> Self {
		self.saturating_add(other)
	}
	#[inline]
	fn saturating_sub(self, other: Self) -> Self {
		self.saturating_sub(other)
	}
}
