// Copyright © 2021-2024 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{Select, SimdBits};
use core::simd::{
	cmp::{SimdPartialEq, SimdPartialOrd},
	num::SimdUint,
	LaneCount, Mask, Simd, SupportedLaneCount,
};

impl<const N: usize> SimdBits<u32, N> for Simd<u32, N>
where
	LaneCount<N>: SupportedLaneCount,
{
	type Mask = Mask<i32, N>;

	#[inline]
	fn splat(value: u32) -> Self {
		Self::splat(value)
	}

	#[inline]
	fn as_simd(slice: &[u32]) -> (&[u32], &[Self], &[u32]) {
		slice.as_simd()
	}

	#[inline]
	fn as_simd_mut(slice: &mut [u32]) -> (&mut [u32], &mut [Self], &mut [u32]) {
		slice.as_simd_mut()
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

impl<const N: usize> Select<Mask<i32, N>> for Simd<u32, N>
where
	LaneCount<N>: SupportedLaneCount,
{
	#[inline]
	fn select(mask: Mask<i32, N>, true_values: Self, false_values: Self) -> Self {
		mask.select(true_values, false_values)
	}
}
