// Copyright © 2021-2022 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{Select, SimdBits};
use core::simd::{
	LaneCount, Mask, Simd, SimdPartialEq, SimdPartialOrd, SimdUint, SupportedLaneCount,
};

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
	fn simd_eq(self, other: Self) -> Self::Mask {
		SimdPartialEq::simd_eq(self, other)
	}
	#[inline]
	fn simd_ne(self, other: Self) -> Self::Mask {
		SimdPartialEq::simd_ne(self, other)
	}
	#[inline]
	fn simd_lt(self, other: Self) -> Self::Mask {
		SimdPartialOrd::simd_lt(self, other)
	}
	#[inline]
	fn simd_gt(self, other: Self) -> Self::Mask {
		SimdPartialOrd::simd_gt(self, other)
	}
	#[inline]
	fn simd_le(self, other: Self) -> Self::Mask {
		SimdPartialOrd::simd_le(self, other)
	}
	#[inline]
	fn simd_ge(self, other: Self) -> Self::Mask {
		SimdPartialOrd::simd_ge(self, other)
	}

	#[inline]
	fn saturating_add(self, other: Self) -> Self {
		SimdUint::saturating_add(self, other)
	}
	#[inline]
	fn saturating_sub(self, other: Self) -> Self {
		SimdUint::saturating_sub(self, other)
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
