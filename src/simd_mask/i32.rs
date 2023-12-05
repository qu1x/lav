// Copyright © 2021-2022 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{Select, SimdMask};
use core::simd::{LaneCount, Mask, SupportedLaneCount};

impl<const N: usize> SimdMask<N> for Mask<i32, N>
where
	LaneCount<N>: SupportedLaneCount,
{
	#[inline]
	fn splat(value: bool) -> Self {
		Self::splat(value)
	}

	#[inline]
	fn from_array(array: [bool; N]) -> Self {
		Self::from(array)
	}
	#[inline]
	fn to_array(self) -> [bool; N] {
		self.to_array()
	}

	#[inline]
	fn all(self) -> bool {
		self.all()
	}
	#[inline]
	fn any(self) -> bool {
		self.any()
	}

	#[inline]
	fn set(&mut self, lane: usize, value: bool) {
		self.set(lane, value);
	}
	#[inline]
	fn test(&self, lane: usize) -> bool {
		self.test(lane)
	}
}

impl<const N: usize> Select<Self> for Mask<i32, N>
where
	LaneCount<N>: SupportedLaneCount,
{
	#[inline]
	fn select(mask: Self, true_values: Self, false_values: Self) -> Self {
		mask.select_mask(true_values, false_values)
	}
}
