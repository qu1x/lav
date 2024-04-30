// Copyright © 2021-2024 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::Bits;
use core::simd::{LaneCount, Simd, SupportedLaneCount};

impl Bits for u64 {
	type Simd<const N: usize> = Simd<Self, N>
	where
		LaneCount<N>: SupportedLaneCount;

	const MIN: Self = Self::MIN;
	const MAX: Self = Self::MAX;

	const ONE: Self = 1;

	#[inline]
	fn saturating_add(self, other: Self) -> Self {
		self.saturating_add(other)
	}
	#[inline]
	fn saturating_sub(self, other: Self) -> Self {
		self.saturating_sub(other)
	}

	#[inline]
	fn as_simd<const N: usize>(slice: &[Self]) -> (&[Self], &[Self::Simd<N>], &[Self])
	where
		LaneCount<N>: SupportedLaneCount,
	{
		slice.as_simd()
	}

	#[inline]
	fn as_simd_mut<const N: usize>(
		slice: &mut [Self],
	) -> (&mut [Self], &mut [Self::Simd<N>], &mut [Self])
	where
		LaneCount<N>: SupportedLaneCount,
	{
		slice.as_simd_mut()
	}
}
